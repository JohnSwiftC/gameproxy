# GameProxy

A quick and easy HTTP web proxy that does not rely on seperate servers to run. Easily works around coorporation or school site filtering!

Hides traffic with a fake home page.

# Build

Build with `cargo build --release`.

When executing, ensure your current working directory is that of the `connect.html` file.

# Usage

Visiting the route `/resetroute` will reset the proxy and it will present the fake page.

Visiting the route `/changesite/{your website}` will configure the proxy to point towards that address.

# Current Problems

> These problems emerge because of the very specific restrictions I have on myself. A user is unable to tunnel an IP in the browser,
> and the user is also unable to configure real proxy settings on their machine for my usecase.

There is currently no way for the proxy to intercept requests made directly to the domain name by the website. For example, YouTube might make requests to their CDN directly, and these requests will not be passed to the proxy.

The proxy server can maintain only one site at a time for all of its users.

Just like any other website, an organization can still see all of your traffic if they use some sort of MITM. Think mitmproxy and a custom Root CA for HTTPS monitoring.
