using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
namespace ThinkThen;
public static class ThinkThenRegistration
{
    public static IServiceCollection AddThinkThen(this IServiceCollection services, Action<ThinkThenOptions> configure)
    {
        services.AddOptions<ThinkThenOptions>().Configure(configure);
        services.AddSingleton(provider => new ThinkThenClient(provider.GetRequiredService<IOptions<ThinkThenOptions>>().Value,
            provider.GetService<ILogger<ThinkThenClient>>()));
        return services;
    }
}
