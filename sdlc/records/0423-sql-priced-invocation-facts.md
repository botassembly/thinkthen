# 0423: Explain SQL pricing and timing limits

SQL documentation now distinguishes input-token estimates and shared usage totals from isolated price and engine time. Clients measure statement wall time and price complete provider usage outside SQL; missing usage or tariffs remain unknown. The dbt v1 profile uses a project-owned extension directory so installation does not collide with another origin. No SQL function or result shape changed.

One fresh High source review accepted the documentation. The full public site build passed. Full tests and lint run on the landing commit. The larger SQL priced-facts API remains deferred.
