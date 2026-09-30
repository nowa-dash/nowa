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
        implicitHeight: 30
        color: "#1a1b26"

        // Launcher button
        Rectangle {
            id: launchButton

            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.leftMargin: 6
            width: 70
            height: 22
            radius: 4
            color: mouse.containsMouse ? "#414868" : "#292e42"

            Text {
                anchors.centerIn: parent
                text: "Launcher"
                color: "#a9b1d6"
                font.pixelSize: 12
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
            text: "My First Bar"
            color: "#a9b1d6"
            font.pixelSize: 14
        }

        Rectangle {
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            anchors.rightMargin: 6
            width: 34
            height: 22
            radius: 4
            visible: ToplevelManager.activeToplevel !== null
            color: closeMouse.containsMouse ? "#7f3b3b" : "#3b2b2b"

            Text {
                anchors.centerIn: parent
                text: ""
                color: "#f5c2c2"
            }

            MouseArea {
                id: closeMouse

                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    if (ToplevelManager.activeToplevel) {
                        ToplevelManager.activeToplevel.close();
                    }
                }
            }

        }

    }

}
