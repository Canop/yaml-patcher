[![MIT][s2]][l2] [![Latest Version][s1]][l1]

[s1]: https://img.shields.io/crates/v/yaml-patcher.svg
[l1]: https://crates.io/crates/yaml-patcher

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

A generic patcher for YAML files.

Given a base file and a patch file, yaml-patcher will output a modified file.

The patch file is a list of (`path`, `value`) where

- `path` is a space separated list of map keys or array indexes forming a path to the value
- `value` is any kind of YAML value, even structured, and will replace the old value

# Usage

```bash
yaml-patcher --base base.yml --patch patch.yml > patched.yml
```

# Example

This is a simplified example for a real use-case: modifying a Mkdocs file in a build chain.

## Base file

```yaml
site_name: OurApp
site_description: 'App Usage & Configuration'
site_url: https://oursite.com/our-app/
dev_addr: '127.0.0.1:8666'
nav:
        - Usage:
                - Modules & Menus: menus-modules.md
                - Product Finder: product-finder.md
                - Product Viewer: product-viewer.md
                - Rules Editor: ed.md
                - Flow Editor: flow-editor.md
        - Concepts:
                - Flakes: flakes.md
                - Product Genealogy: tracking.md
                - TRT Flows: trt.md
                - Scheduling: scheduling.md
        - Configuration:
                - Installation: install.md
                - World: world.md
                - KB: kb.md
                - Resc: resc.md
                - Nginx: nginx.md
                - Internationalization: i18n.md
extra_javascript:
        - js/autoclose-tree.js
        - js/filter.js
```

## Patch file

```yaml
site_description: 'Vroum Vroum'
site_url: http://ourclient.com/some/path
nav 0 Usage:
        - Product Finder: product-finder.md
        - Product Viewer: product-viewer.md
nav 2 Configuration:
        - World: world.md
        - KB: kb.md
```

## Result

```bash
yaml-patcher -b mkdocs.yml -p our-client-patch.yml > our-client/mkdocs.yml
```

```yaml
site_name: OurApp
edit_uri: ""
dev_addr: "127.0.0.1:8666"
nav:
    - Usage:
         - Modules & Menus: menus-modules.md
         - Product Finder: product-finder.md
         - Product Viewer: product-viewer.md
    - Concepts:
         - Flakes: flakes.md
         - Product Genealogy: tracking.md
         - TRT Flows: trt.md
         - Scheduling: scheduling.md
    - Configuration:
         - World: world.md
         - KB: kb.md
extra_javascript:
    - js/autoclose-tree.js
    - js/filter.js
site_description: Vroum Vroum
site_url: "http://some/other/url"
```

# Features

Right now yaml-patcher only change existing values to new values, because this covers my current needs.

I might add new features, like removing values, or adding them, or applying pattern based replacements.

If you need such feature please tell me.
