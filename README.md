Patch Ultra-Wide Screen for The Outer Worlds
============================================

This patch modifies the game to work better in 21:9 aspect ratio. 32:9 is not
supported at the moment (help requested).

What does it do?
----------------

 *  The FOV will be calculated based on a 16:9 screen. (The FOV is still
    horizontal).
 *  It removes the pillarboxing from the cutscenes and dialog screens.

Usage
-----

1.  Go the the "Releases" tab on the GitHub page and download the latest
    executable or the source code if you want to compile the program yourself.

1.  Copy the executable in the game directory under:

    ```
    C:\Program Files\Epic Games\TheOuterWorlds\Indiana\Binaries\Win64
    ```

    (This is the default directory, it might be located somewhere else on your
    machine)

2.  Run the patch, it should display something like this:

    ```
    File path: IndianaEpicGameStore-Win64-Shipping.exe
    Backup path: IndianaEpicGameStore-Win64-Shipping.bak
    Replacement succeeded.
    Replacement succeeded.
    Proceed? [y/n]
    ```

    Type `y` and hit ENTER to confirm.

    *Notes:*
     *  If there are any WARNING, it is possible that this patch won't work
     *  If you run the patch multiple times it will fail. You can revert the
        change by renaming the file `IndianaEpicGameStore-Win64-Shipping.bak`
        to `IndianaEpicGameStore-Win64-Shipping.exe` (after having deleted the
        exe file).

Disclaimer
----------

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
