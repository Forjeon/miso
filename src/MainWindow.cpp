#include <QLabel>
#include<QVBoxLayout>

#include "MainWindow.h"

MainWindow::MainWindow(QWidget* parent) : QMainWindow(parent) {
	QWidget* centralWidget = new QWidget(this);
	setCentralWidget(centralWidget);

	QLabel* label = new QLabel("Hello World!", this);
	label->setAlignment(Qt::AlignCenter);

	QVBoxLayout* layout = new QVBoxLayout(centralWidget);
	layout->addWidget(label);

	resize(400, 300);
}

MainWindow::~MainWindow() {}
