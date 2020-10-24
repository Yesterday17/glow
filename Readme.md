# Glow

Glow is the core of [Project Glow][project-glow-intro].

[project-glow-intro]: https://blog.mmf.moe/post/project-glow-start/

## Organization

- crate [glow-common](./glow-common) provides common parts used in other crates, such as traits.
- crate [glow-utils](./glow-utils) provides some useful utilities.
- crate [glow-core](./glow-core) implements main features of glow.
  - DNS: simple dns client with compression support to bypass some firewalls.
- crate [glow](./glow) is the final binary product.