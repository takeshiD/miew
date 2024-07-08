# miew
markdown preview tool

# Features
* Selectable parser, render, destination(browser, terminal, etc...)
* Support mermaid-syntax code block
* 

# Installation
sorry not yet prepared

# Usage

## Rendering to 24bit color ascii text

This mode is default.

```bash
$ miew ./example.md

Rendered text
```

> Todo: add image


## Rendering to HTML

This mode is optional.

```bash
$ miew --render=html ./example.md
<div>
    <pre>...</pre>
    <img src="..." />
    ...
</div>
```

