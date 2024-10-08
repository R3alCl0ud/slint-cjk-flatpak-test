# slint-cjk-flatpak-test

Repo containing the smallest version of this bug

### Instructions
```bash
flatpak install org.flatpak.Builder org.freedesktop.Platform//24.08 org.freedesktop.Sdk//24.08
cd flatpak
./build-flatpak.sh
./run-winit-backend.sh # glyphs do not render properly
./run-qt-backend.sh # glyphs render properly
```
