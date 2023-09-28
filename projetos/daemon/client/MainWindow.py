from PySide6.QtWidgets import QMainWindow, QApplication, QVBoxLayout, QHBoxLayout, QWidget, QTableWidget, QTableWidgetItem, QScrollArea, QComboBox, QPushButton, QLabel
from PySide6.QtCore import Qt
import time

class MainWindow(QMainWindow):
    def __init__(self, socket, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Super DAW")
        self.resize(400, 100)

        self.__socket = socket
        self.__currNotes = []

        # layout principal - vertical
        layout = QVBoxLayout()
        # cria um scroll area
        scroll = QScrollArea()
        # set scroll to fit its elements
        scroll.setWidgetResizable(True)
        # crate single row, 8 column empty table
        # remove all headers
        self.__table = QTableWidget(1, 8)
        self.__table.horizontalHeader().setVisible(False)
        self.__table.verticalHeader().setVisible(False)
        # non editable
        self.__table.setEditTriggers(QTableWidget.NoEditTriggers)
        # non selectable
        self.__table.setSelectionMode(QTableWidget.NoSelection)
        # non clickable
        self.__table.setFocusPolicy(Qt.NoFocus)
        # table height is 50px
        self.__table.setFixedHeight(50)
        # each column has 50px width
        for i in range(8):
            self.__table.setColumnWidth(i, 50)
        # set to scroll
        scroll.setWidget(self.__table)
        # scroll's height should be the same as table's height
        scroll.setMinimumHeight(self.__table.height())
        # adicionador de notas
        # primeiro, o layout
        add_note_layout = QHBoxLayout()
        # agora, o combo box de notas
        self.__note_combo_box = QComboBox()
        # adiciona as notas
        notes = {
            'C': 'c',
            'C#': '1',
            'D': 'd',
            'D#': '2',
            'E': 'e',
            'F': 'f',
            'F#': '3',
            'G': 'g',
            'G#': '4',
            'A': 'a',
            'A#': '5',
            'B': 'b'
        }
        for note in notes:
            self.__note_combo_box.addItem(note, notes[note])
        # agora o botão de adicionar
        add_note_button = QPushButton('Adicionar nota')
        add_note_button.clicked.connect(self.add_note)
        play_button = QPushButton('Tocar  tudo')
        play_button.clicked.connect(self.play_notes)
        # adiciona no layout...
        add_note_layout.addWidget(self.__note_combo_box)
        add_note_layout.addWidget(add_note_button)
        add_note_layout.addWidget(play_button)
        # status label
        self.__statusLabel = QLabel('Adicione notas para tocar!')
        # adiciona tudo no layout
        layout.addWidget(scroll)
        layout.addLayout(add_note_layout)
        layout.addWidget(self.__statusLabel)
        # layout principal - central widget
        central_widget = QWidget()
        central_widget.setLayout(layout)
        self.setCentralWidget(central_widget)

    def add_note(self):
        # pega a nota selecionada
        note = self.__note_combo_box.currentData()
        noteName = self.__note_combo_box.currentText()
        self.__currNotes.append(note)
        if len(self.__currNotes) > 8:
            self.__table.setColumnCount(len(self.__currNotes))
        # update table with note list
        # each note is a column
        self.__table.setItem(0, len(self.__currNotes) - 1, QTableWidgetItem(noteName))

    def play_notes(self):
        self.__statusLabel.setText('Tocando...')
        QApplication.processEvents()
        for note in self.__currNotes:
            self.__socket.send(note.encode())
            response = self.__socket.recv(1024).decode()
            if response != 'ok':
                print(f"Possível erro: {response}")
                return
            time.sleep(0.1)
        self.__statusLabel.setText('Adicione notas para tocar!')
        
    def closeEvent(self, event):
        self.__socket.close()
        super().closeEvent(event)