import QtQuick
import QtQuick.Layouts
import Quickshell
import Quickshell.Io
import Quickshell.Wayland

PanelWindow {
    id: root

    property var appsList: [{
        "name": "Android Auto",
        "icon": "",
        "exec": "autoapp"
    }, {
        "name": "Files",
        "icon": "󰉋",
        "exec": "nemo"
    }, {
        "name": "Terminal",
        "icon": "󰞷",
        "exec": "foot"
    }, {
        "name": "Power",
        "icon": "󰐥",
        "items": [{
            "name": "Shutdown",
            "icon": "󰐥",
            "exec": "shutdown now"
        }, {
            "name": "Reboot",
            "icon": "󰜉",
            "exec": "reboot"
        }]
    }]
    property var folderStack: []
    readonly property var currentItems: folderStack.length > 0 ? folderStack[folderStack.length - 1].items : appsList
    readonly property var visibleItems: folderStack.length > 0 ? [{
        "name": "Back",
        "icon": "󰌑",
        "Return": true
    }].concat(currentItems) : currentItems

    function launchApp(app) {
        if (app && app.exec) {
            appProcess.command = ["bash", "-c", app.exec];
            appProcess.running = true;
            root.visible = false;
        }
    }

    function openItem(item) {
        if (item.Return)
            folderStack = folderStack.slice(0, -1);
        else if (item.items)
            folderStack = folderStack.concat([item]);
        else
            launchApp(item);
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

    Rectangle {
        anchors.fill: parent
        color: "#1e1e2e"
        border.color: "#313244"

        GridView {
            id: appListView

            anchors.fill: parent
            anchors.margins: 5
            clip: true
            focus: root.visible
            model: root.visibleItems
            cellWidth: Math.floor(appListView.width / 3)
            cellHeight: Math.floor(appListView.height / 2)

            delegate: Rectangle {
                required property var modelData

                width: appListView.cellWidth
                height: appListView.cellHeight
                border.color: "#1E1E2E"
                border.width: 5
                color: itemMouse.pressed ? "#585b70" : (itemMouse.containsMouse ? "#45475a" : "#313244")

                ColumnLayout {
                    anchors.centerIn: parent

                    Text {
                        text: modelData.icon
                        font.pixelSize: 78
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
                    onClicked: root.openItem(modelData)
                }

            }

        }

    }

}
