# lang2lua

`lang2lua` converts legacy Hyprland `.conf` syntax into native Lua configuration.
Its mappings target **Hyprland 0.57.2**, verified against the official example,
wiki, and generated `hl.meta.lua` stubs.

```sh
lang2lua hyprland.conf hyprland.lua
lang2lua --check keybinds.conf keybinds.lua

# Piping still works
cat general.conf | cargo run -- --offline > general.lua
```

Convert the root file and every sourced file. Legacy variables shared across `source`
boundaries are kept in `_hyprlang_vars`, so the generated root Lua must assign them
before its converted modules are loaded.

The converter refuses to use the same file for input and output. Review warnings go
to stderr, and `--check` exits with status 2 if anything needs manual conversion.
By default it makes a two-second request to Hyprland's official GitHub latest-release
URL and warns when its ruleset is stale; `--offline` disables that request. No config
content is sent over the network.

Supported in the first release:

- nested `hl.config` sections and typed scalar values
- variables, `source`, monitors, environment variables, and autostart commands
- Bezier curves, animations, gestures, and match-style window/layer rules
- the common bind families and dispatchers exercised by the sanitized test cases

Unsupported syntax is emitted only as a Lua comment and reported for manual review.
Plugin-specific directives are intentionally not guessed because their Lua APIs vary
with plugin versions.

Compatibility references: the official [v0.57.2 release](https://github.com/hyprwm/Hyprland/releases/tag/v0.57.2),
[shipped Lua example](https://github.com/hyprwm/Hyprland/blob/v0.57.2/example/hyprland.lua),
and [configuration wiki](https://wiki.hypr.land/configuring/core/).

## Privacy

Tests use small synthetic examples derived from real conversions. Do not commit raw
dotfiles: they can contain usernames, home paths, monitor/device identifiers, commands,
and precise locations. The personal source configs used during development remain in
`~/.config/hypr` and are not copied into this repository.

`tests/fixtures/minimal.conf` is a sanitized real-world fixture with its exact expected
Lua output, covering nested settings, colors, gestures, variables, and a keybind.

## Updating the ruleset

Compare against the latest official Hyprland example, wiki, and generated
`/usr/share/hypr/stubs/hl.meta.lua`, update `HYPRLAND_RULESET`, then add a focused test
for every changed mapping. If the startup check reports a newer Hyprland release,
reach out to [@vanshxgoel](https://twitter.com/vanshxgoel) on Twitter/X and ask them to
take a look.

## License

MIT

