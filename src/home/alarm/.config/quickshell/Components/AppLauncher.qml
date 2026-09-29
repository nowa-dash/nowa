import QtQuick
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Wayland

PanelWindow {
    id: root

    property var appsList: [{
        "name": "Android Auto",
        "exec": "autoapp",
        "icon": ""
    }, {
        "name": "Files",
        "exec": "nemo",
        "icon": "󰉋"
    }, {
        "name": "Terminal",
        "exec": "foot",
        "icon": "󰞷"
    }]

    function launchApp(app) {
        if (app && app.exec) {
            appProcess.command = ["bash", "-c", app.exec];
            appProcess.running = true;
            root.visible = false;
        }
    }

    implicitWidth: 500
    implicitHeight: 400
    color: "transparent"
    visible: false // was: true
    WlrLayershell.keyboardFocus: WlrKeyboardFocus.Exclusive
    WlrLayershell.layer: WlrLayer.Overlay // draw above the bar and other windows

    anchors {
        top: false
        bottom: false
        left: false
        right: false
    }

    Process {
        id: appProcess
    }

    Shortcut {
        sequence: "Escape"
        onActivated: root.visible = false
    }

    Rectangle {
        anchors.fill: parent
        color: "#1e1e2e"
        radius: 12
        border.color: "#313244"
        border.width: 1

        ListView {
            id: appListView

            anchors.fill: parent
            anchors.margins: 12
            spacing: 4
            clip: true
            focus: root.visible
            model: root.appsList

            delegate: Rectangle {
                required property var modelData

                width: appListView.width
                height: 42
                radius: 6
                color: itemMouse.containsMouse ? "#45475a" : "transparent"

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 12
                    anchors.rightMargin: 12
                    spacing: 12

                    Text {
                        text: modelData.icon
                        font.pixelSize: 18
                        color: "#89b4fa"
                    }

                    Text {
                        text: modelData.name
                        font.pixelSize: 14
                        color: "#cdd6f4"
                        Layout.fillWidth: true
                    }

                }

                MouseArea {
                    id: itemMouse

                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.launchApp(modelData)
                }

            }

        }

    }

}
