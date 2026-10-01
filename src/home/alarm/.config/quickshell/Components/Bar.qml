import QtQuick
import Quickshell
import Quickshell.Wayland

ShellRoot {
    AppLauncher {
        id: launcher

        visible: false // Hidden by default
    }

    PanelWindow {
        id: bar

        anchors.top: true
        anchors.left: true
        anchors.right: true
        implicitHeight: 60
        color: "#1a1b26"

        // Launcher button
        Rectangle {
            id: launchButton

            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            width: parent.height // Yes, this is supposed to be height. This way the button stays square and scales with the bar.
            height: parent.height
            radius: 4
            color: mouse.containsMouse ? "#414868" : "#292e42"

            Text {
                anchors.centerIn: parent
                text: "󰍜"
                color: "#a9b1d6"
                font.pixelSize: 50
            }

            MouseArea {
                id: mouse

                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: launcher.visible = !launcher.visible
            }

        }

        Text {
            anchors.centerIn: parent
            text: "Hello, World!"
            color: "#a9b1d6"
            font.pixelSize: 24
        }

        Rectangle {
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: 6
            width: 34
            height: 22
            radius: 4
            visible: ToplevelManager.activeToplevel !== null && !launcher.visible
            color: closeMouse.containsMouse ? "#7f3b3b" : "#3b2b2b"

            Text {
                anchors.centerIn: parent
                text: ""
                color: "#f5c2c2"
            }

            MouseArea {
                id: closeMouse

                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    if (ToplevelManager.activeToplevel)
                        ToplevelManager.activeToplevel.close();

                }
            }

        }

    }

}
