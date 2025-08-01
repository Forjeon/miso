#ifndef MISO_MAIN_WINDOW_H
#define MISO_MAIN_WINDOW_H

#include <QMainWindow>

class MainWindow : public QMainWindow {
	Q_OBJECT

public:
	MainWindow(QWidget* parent = nullptr);
	~MainWindow();
};

#endif
