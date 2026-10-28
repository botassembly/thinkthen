# frozen_string_literal: true

module ThinkThen
  module NativeAllocator
    def self.prepare
      return unless FFI::Platform::IS_LINUX
      flags = FFI::DynamicLibrary::RTLD_LAZY | FFI::DynamicLibrary::RTLD_LOCAL
      @library = FFI::DynamicLibrary.open(FFI::Library::LIBC, flags)
      return unless @library.find_function("gnu_get_libc_version")
      process = FFI::DynamicLibrary.open(nil, flags)
      return unless %w[malloc calloc realloc free].all? do |name|
        active = process.find_function(name)
        expected = @library.find_function(name)
        active && expected && active.address == expected.address
      end
      function = @library.find_function("malloc_trim")
      @trim = FFI::Function.new(:int, [:size_t], function) if function
    rescue LoadError, FFI::NotFoundError
      @trim = nil
    end
    private_class_method :prepare

    def self.trim
      @trim&.call(0)
      nil
    rescue StandardError
      # Reclaiming free pages must not interrupt an already completed release.
      nil
    end

    @trim = nil
    prepare
  end
  private_constant :NativeAllocator
end
