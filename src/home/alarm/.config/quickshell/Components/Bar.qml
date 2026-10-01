import QtQuick
import Quickshell
import Quickshell.Io
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

        Process {
            id: tempProc

            // Get the temp
            command: ["sh", "-c", "cat $(grep -lE 'coretemp|k10temp|cpu_thermal' /sys/class/hwmon/hwmon*/name | sed 's/name/temp1_input/' | head -1) /sys/class/thermal/thermal_zone0/temp | head -1"]

            stdout: StdioCollector {
                onStreamFinished: tempText.text = Math.round(parseInt(text) / 1000) + "°C"
            }

        }

        Timer {
            interval: 2000 // Update every 2 seconds
            running: true
            repeat: true
            triggeredOnStart: true
            onTriggered: tempProc.running = true
        }

        // Launcher button
        Rectangle {
            id: launchButton

            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            width: parent.height // Yes, this is supposed to be height. This way the button stays square and scales with the bar.
            height: parent.height
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
            id: tempText

            anchors.centerIn: parent
            font.pixelSize: 24
            color: "#a9b1d6"
        }

        Rectangle {
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            width: parent.height // Again... This is supposed to be height so it scales.
            height: parent.height
            visible: ToplevelManager.activeToplevel !== null && !launcher.visible
            color: closeMouse.containsMouse ? "#7f3b3b" : "#3b2b2b"

            Text {
                anchors.centerIn: parent
                text: "󰅖"
                font.pixelSize: 50
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
