#./gensources-flatpak.sh
rm -rf ./slint-test-repo
mkdir ./slint-test-repo
flatpak run org.flatpak.Builder --force-clean --user --install --ccache --mirror-screenshots-url=https://dl.flathub.org/media/ --repo=slint-test-repo repo io.github.r3alcl0ud.slint-cjk-flatpak-test.json



flatpak build-bundle ./slint-test-repo slint-cjk-flatpak-test.flatpak io.github.r3alcl0ud.slint-cjk-flatpak-test

