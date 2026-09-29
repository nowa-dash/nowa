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

    function launchCurrent() {
        if (appListView.currentIndex >= 0 && appListView.currentIndex < appsList.length) {
            let app = appsList[appListView.currentIndex];
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
    // Reset selection and focus the list every time it's opened
    onVisibleChanged: {
        if (visible) {
            appListView.currentIndex = 0;
            appListView.forceActiveFocus();
        }
    }

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

    Shortcut {
        sequence: "Return"
        onActivated: root.launchCurrent()
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
            focus: true
            model: root.appsList
            currentIndex: 0
            Keys.onDownPressed: incrementCurrentIndex()
            Keys.onUpPressed: decrementCurrentIndex()

            delegate: Rectangle {
                required property var modelData
                required property int index

                width: appListView.width
                height: 42
                radius: 6
                color: index === appListView.currentIndex ? "#45475a" : "transparent"

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
                    anchors.fill: parent
                    onClicked: {
                        appListView.currentIndex = index;
                        root.launchCurrent();
                    }
                }

            }

        }

    }

}
