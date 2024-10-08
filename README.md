# slint-cjk-flatpak-test

Repo containing the smallest version of this bug

### Instructions
```bash
flatpak install org.flatpak.Builder org.kde.Platform//6.7 org.kde.Sdk//6.7
cd flatpak
./build-flatpak.sh
./run-winit-backend.sh # glyphs do not render properly
./run-qt-backend.sh # glyphs render properly
```
