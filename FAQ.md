# FAQ

## Visualization framework problems

### xcrun: error: unable to find utility "metal", not a developer tool or in PATH
This error is generally caused by a missing installation of Xcode on your system,
necessary for Amethyst (which the visualization framework is based on) to correctly
compile the metal shaders needed for the presentation. It can also happen for
outdated versions of Xcode, try updating to the latest version to solve the problem.

### No package 'alsa' found
The visualization requires some dependencies on linux distributions.
Check [this link](https://github.com/amethyst/amethyst#dependencies) and follow the steps
for your distribution.

### vulkan: No DRI3 support detected - required for presentation.
This is caused by a missing option related to DRI 3 support, usually caused by Intel graphics cards.
Edit your `/etc/X11/xorg.conf.d/20-intel.conf` file and add `Option      "DRI"    "3"`
in your device section, as shown in the example [here](https://wiki.archlinux.org/index.php/Vulkan#Error_-_vulkan:_No_DRI3_support).

