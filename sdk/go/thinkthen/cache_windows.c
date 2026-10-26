#include <windows.h>
#include <aclapi.h>
#include <stdlib.h>

/* Only the caller and the operating system's administrators may modify the
 * cache. Reparse points and unsupported allow-ACE forms fail closed. */
int tt_cache_secure(const char *path, int directory) {
    int result = 0;
    HANDLE token = NULL;
    TOKEN_USER *user = NULL;
    PSECURITY_DESCRIPTOR descriptor = NULL;
    PSID owner = NULL;
    PACL acl = NULL;
    DWORD size = 0;
    BYTE system[SECURITY_MAX_SID_SIZE], administrators[SECURITY_MAX_SID_SIZE];
    DWORD system_size = sizeof(system), admin_size = sizeof(administrators);
    int count = MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, path, -1, NULL, 0);
    if (!count) return 0;
    wchar_t *wide = malloc((size_t)count * sizeof(*wide));
    if (!wide) return 0;
    if (!MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, path, -1, wide, count)) goto done;
    DWORD attributes = GetFileAttributesW(wide);
    if (attributes == INVALID_FILE_ATTRIBUTES || (attributes & FILE_ATTRIBUTE_REPARSE_POINT)
            || (!!(attributes & FILE_ATTRIBUTE_DIRECTORY) != !!directory)) goto done;
    if (GetNamedSecurityInfoW(wide, SE_FILE_OBJECT, OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &owner, NULL, &acl, NULL, &descriptor) != ERROR_SUCCESS || !acl) goto done;
    if (!OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &token)) goto done;
    GetTokenInformation(token, TokenUser, NULL, 0, &size);
    user = malloc(size);
    if (!user || !GetTokenInformation(token, TokenUser, user, size, &size)
            || !EqualSid(owner, user->User.Sid)) goto done;
    if (!CreateWellKnownSid(WinLocalSystemSid, NULL, system, &system_size)
            || !CreateWellKnownSid(WinBuiltinAdministratorsSid, NULL, administrators, &admin_size)) goto done;
    for (DWORD index = 0; index < acl->AceCount; ++index) {
        ACE_HEADER *header;
        if (!GetAce(acl, index, (void **)&header)) goto done;
        if (header->AceFlags & INHERIT_ONLY_ACE) continue;
        if (header->AceType == ACCESS_DENIED_ACE_TYPE) continue;
        if (header->AceType != ACCESS_ALLOWED_ACE_TYPE) goto done;
        ACCESS_ALLOWED_ACE *ace = (ACCESS_ALLOWED_ACE *)header;
        DWORD writes = GENERIC_WRITE | GENERIC_ALL | FILE_WRITE_DATA | FILE_APPEND_DATA
            | FILE_WRITE_EA | FILE_WRITE_ATTRIBUTES | FILE_DELETE_CHILD | DELETE | WRITE_DAC | WRITE_OWNER;
        PSID principal = (PSID)&ace->SidStart;
        if ((ace->Mask & writes) && !EqualSid(principal, owner)
                && !EqualSid(principal, system) && !EqualSid(principal, administrators)) goto done;
    }
    result = 1;
done:
    if (descriptor) LocalFree(descriptor);
    if (token) CloseHandle(token);
    free(user);
    free(wide);
    return result;
}
