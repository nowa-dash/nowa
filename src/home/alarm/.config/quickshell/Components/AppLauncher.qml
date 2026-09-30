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
    },{
        "name": "Power",
        "exec": "",
        "icon": "󰐥" 
    }]

    function launchApp(app) {
        if (app && app.exec) {
            appProcess.command = ["bash", "-c", app.exec];
            appProcess.running = true;
            root.visible = false;
        }
    }

    visible: false
    exclusionMode: ExclusionMode.Ignore
    margins.top: bar.height

    anchors {
        top: true
        bottom: true
        left: true
        right: true
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
        border.color: "#313244"
        border.width: 1

        GridView {
            id: appListView

            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            anchors.margins: 12
            clip: true
            focus: root.visible
            model: root.appsList
            cellWidth: Math.floor(appListView.width / 3)
            cellHeight: Math.floor(appListView.height / 2)

            delegate: Rectangle {
                required property var modelData

                width: appListView.cellWidth - 12
                height: appListView.cellHeight - 12
                radius: 10
                color: itemMouse.pressed ? "#585b70" : (itemMouse.containsMouse ? "#45475a" : "#313244")

                ColumnLayout {
                    anchors.centerIn: parent
                    spacing: 10

                    Text {
                        text: modelData.icon
                        font.pixelSize: 48
                        color: "#89b4fa"
                        Layout.alignment: Qt.AlignHCenter
                    }

                    Text {
                        text: modelData.name
                        font.pixelSize: 20
                        color: "#cdd6f4"
                        horizontalAlignment: Text.AlignHCenter
                        Layout.alignment: Qt.AlignHCenter
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
