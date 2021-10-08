#!/bin/bash

set -euo pipefail

echo "Compiling cdylib..."
cargo build --release

echo "Creating loadable bundle for MacOS..."
rm -rf ./AndrewVstExperiments.vst/
./osx_vst_bundler.sh AndrewVstExperiments target/release/libvst_experiments.dylib

echo "Copying bundle into the main VST directory... (you may be prompted for your password)"
sudo rm -rf /Library/Audio/Plug-Ins/VST/AndrewVstExperiments.vst/
sudo cp -r AndrewVstExperiments.vst /Library/Audio/Plug-Ins/VST/

echo "Done!"
