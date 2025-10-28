import sys
import requests
from PyQt5.QtWidgets import (QApplication, QMainWindow, QWidget, QVBoxLayout, 
                             QHBoxLayout, QLabel, QPushButton, QDoubleSpinBox,
                             QComboBox, QGroupBox, QGridLayout)
from PyQt5.QtCore import QTimer, Qt
from PyQt5.QtGui import QFont
import pyqtgraph as pg

class SignalGUI(QMainWindow):
    def __init__(self):
        super().__init__()
        
        self.setWindowTitle("Signal Processor")
        self.setGeometry(50, 50, 1400, 700)
        
        # Central widget
        central = QWidget()
        self.setCentralWidget(central)
        main_layout = QHBoxLayout(central)
        
        # LEFT: Controls
        left = self.create_controls()
        main_layout.addWidget(left, 1)
        
        # RIGHT: Plots
        right = self.create_plots()
        main_layout.addWidget(right, 3)
        
        # Timer for auto-update
        self.timer = QTimer()
        self.timer.timeout.connect(self.update_data)
        self.running = False
        
        print("GUI sukses running!")
        
    def create_controls(self):
        panel = QWidget()
        layout = QVBoxLayout(panel)
        
        # Title
        title = QLabel("Control Panel")
        title.setStyleSheet("font-size: 16px; font-weight: bold;")
        layout.addWidget(title)
        
        # Signal 1
        g1 = QGroupBox("Signal X1")
        g1_layout = QGridLayout()
        g1_layout.addWidget(QLabel("Amp:"), 0, 0)
        self.amp1 = QDoubleSpinBox()
        self.amp1.setRange(0.1, 10)
        self.amp1.setValue(1.0)
        g1_layout.addWidget(self.amp1, 0, 1)
        
        g1_layout.addWidget(QLabel("Freq:"), 1, 0)
        self.freq1 = QDoubleSpinBox()
        self.freq1.setRange(0.1, 100)
        self.freq1.setValue(5.0)
        g1_layout.addWidget(self.freq1, 1, 1)
        
        g1_layout.addWidget(QLabel("Phase:"), 2, 0)
        self.phase1 = QDoubleSpinBox()
        self.phase1.setRange(0, 6.28)
        self.phase1.setValue(0.0)
        g1_layout.addWidget(self.phase1, 2, 1)
        
        g1.setLayout(g1_layout)
        layout.addWidget(g1)
        
        # Signal 2
        g2 = QGroupBox("Signal X2")
        g2_layout = QGridLayout()
        g2_layout.addWidget(QLabel("Amp:"), 0, 0)
        self.amp2 = QDoubleSpinBox()
        self.amp2.setRange(0.1, 10)
        self.amp2.setValue(1.0)
        g2_layout.addWidget(self.amp2, 0, 1)
        
        g2_layout.addWidget(QLabel("Freq:"), 1, 0)
        self.freq2 = QDoubleSpinBox()
        self.freq2.setRange(0.1, 100)
        self.freq2.setValue(10.0)
        g2_layout.addWidget(self.freq2, 1, 1)
        
        g2_layout.addWidget(QLabel("Phase:"), 2, 0)
        self.phase2 = QDoubleSpinBox()
        self.phase2.setRange(0, 6.28)
        self.phase2.setValue(0.0)
        g2_layout.addWidget(self.phase2, 2, 1)
        
        g2.setLayout(g2_layout)
        layout.addWidget(g2)
        
        # Operation
        g3 = QGroupBox("Operation")
        g3_layout = QVBoxLayout()
        self.operation = QComboBox()
        self.operation.addItems(["Add", "Subtract", "Multiply"])
        g3_layout.addWidget(self.operation)
        g3.setLayout(g3_layout)
        layout.addWidget(g3)
        
        # Stats
        g4 = QGroupBox("Statistics")
        g4_layout = QVBoxLayout()
        self.stats_label = QLabel("Max: -\nRMS: -")
        g4_layout.addWidget(self.stats_label)
        g4.setLayout(g4_layout)
        layout.addWidget(g4)
        
        # Buttons
        self.btn_start = QPushButton("START")
        self.btn_start.setStyleSheet("background: green; color: white; padding: 10px; font-weight: bold;")
        self.btn_start.clicked.connect(self.toggle_simulation)
        layout.addWidget(self.btn_start)
        
        self.btn_export = QPushButton("EXPORT CSV")
        self.btn_export.setStyleSheet("background: blue; color: white; padding: 10px;")
        self.btn_export.clicked.connect(self.export_csv)
        layout.addWidget(self.btn_export)
        
        layout.addStretch()
        return panel
        
    def create_plots(self):
        panel = QWidget()
        layout = QVBoxLayout(panel)
        
        # Plot 1
        self.plot1 = pg.PlotWidget(title="Input Signals (X1 & X2)")
        self.plot1.setBackground('w')
        self.plot1.showGrid(x=True, y=True)
        self.curve_x1 = self.plot1.plot(pen='b', width=2)
        self.curve_x2 = self.plot1.plot(pen='c', width=2, style=Qt.DashLine)
        layout.addWidget(self.plot1)
        
        # Plot 2
        self.plot2 = pg.PlotWidget(title="Output Signal (Y)")
        self.plot2.setBackground('w')
        self.plot2.showGrid(x=True, y=True)
        self.curve_y = self.plot2.plot(pen='r', width=2)
        layout.addWidget(self.plot2)
        
        return panel
        
    def toggle_simulation(self):
        if not self.running:
            self.running = True
            self.btn_start.setText("STOP")
            self.btn_start.setStyleSheet("background: red; color: white; padding: 10px; font-weight: bold;")
            self.timer.start(100)
            self.update_data()
            print("Simulasi dimulai")
        else:
            self.running = False
            self.btn_start.setText("START")
            self.btn_start.setStyleSheet("background: green; color: white; padding: 10px; font-weight: bold;")
            self.timer.stop()
            print("Simulasi berhenti")
            
    def update_data(self):
        params = {
            "a1": self.amp1.value(),
            "f1": self.freq1.value(),
            "phi1": self.phase1.value(),
            "a2": self.amp2.value(),
            "f2": self.freq2.value(),
            "phi2": self.phase2.value(),
            "operation": ["add", "subtract", "multiply"][self.operation.currentIndex()],
            "samples": 1000,
            "fs": 1000.0
        }
        
        try:
            response = requests.post("http://127.0.0.1:8080/process", json=params, timeout=1)
            data = response.json()
            
            self.last_data = data
            
            self.curve_x1.setData(data['t'], data['x1'])
            self.curve_x2.setData(data['t'], data['x2'])
            self.curve_y.setData(data['t'], data['y'])
            
            self.stats_label.setText(f"Max: {data['max_amplitude']:.4f}\nRMS: {data['rms']:.4f}")
            
        except Exception as e:
            print(f"Error: {e}")
            
    def export_csv(self):
        if not hasattr(self, 'last_data'):
            print("No data to export")
            return
            
        import csv
        from datetime import datetime
        
        filename = f"signal_{datetime.now().strftime('%Y%m%d_%H%M%S')}.csv"
        
        try:
            with open(filename, 'w', newline='') as f:
                writer = csv.writer(f)
                writer.writerow(['Time', 'X1', 'X2', 'Y'])
                for i in range(len(self.last_data['t'])):
                    writer.writerow([
                        self.last_data['t'][i],
                        self.last_data['x1'][i],
                        self.last_data['x2'][i],
                        self.last_data['y'][i]
                    ])
            print(f"Exported: {filename}")
        except Exception as e:
            print(f"Export error: {e}")

if __name__ == "__main__":
    print("Memulai GUI...")
    print("=" * 60)
    
    app = QApplication(sys.argv)
    app.setFont(QFont("Arial", 10))
    
    window = SignalGUI()
    window.show()
    
    print("GUI window berhasil terlihat!")
    print("=" * 60)
    
    sys.exit(app.exec_())