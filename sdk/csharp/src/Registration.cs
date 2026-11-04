using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
namespace ThinkThen;
/// <summary>Registers a client and options with dependency injection.</summary>
public static class ThinkThenRegistration
{
    /// <summary>Registers a singleton client and configurable options with the service collection.</summary>
    public static IServiceCollection AddThinkThen(this IServiceCollection services, Action<ThinkThenOptions> configure)
    {
        ArgumentNullException.ThrowIfNull(services);
        ArgumentNullException.ThrowIfNull(configure);
        services.AddOptions<ThinkThenOptions>().Configure(configure);
        services.AddSingleton(provider => new ThinkThenClient(provider.GetRequiredService<IOptions<ThinkThenOptions>>().Value,
            provider.GetService<ILogger<ThinkThenClient>>()));
        return services;
    }
}
