import QtQuick
import Quickshell
import Quickshell.Wayland

Variants {
    model: Quickshell.screens

    PanelWindow {
        required property var modelData

        screen: modelData
        WlrLayershell.layer: WlrLayer.Background
        exclusionMode: ExclusionMode.Ignore
        color: "black"

        anchors {
            top: true
            bottom: true
            left: true
            right: true
        }

        Image {
            anchors.fill: parent
            source: "../default_wallpaper.png"
            fillMode: Image.PreserveAspectCrop
            asynchronous: true
            sourceSize.width: parent.width
            sourceSize.height: parent.height
        }

    }

}
