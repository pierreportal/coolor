# Coolor

### Usage:
```
coolor --hex <HEX> [-p <FORMAT>]
```
```
coolor --hex "#f6453a"

Color {
    hex: "#f6453a",
    rgb: Rgb {
        r: 246,
        g: 69,
        b: 58,
    },
    hsl: Hsl {
        h: 3,
        s: 91,
        l: 59.607845,
    },
    lab: Lab {
        l: 56.242218,
        a: 66.08841,
        b: 46.351208,
    },
    luminance_wcag: 0.24154566,
}
```
```
coolor --hex "#f6453a" -p rgb

rgb(246,69,58)
```
```
coolor --hex "#f6453a" -p hsl

hsl(3,91,59.607845)
```
```
coolor --hex "#f6453a" -p lab

lab(56.242218,66.08841,46.351208)
```
```
coolor --hex "#f6453a" -p lum

0.24154566
```
