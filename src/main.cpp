#include <QApplication>
#include "MainWindow.h"

int main(int argc, char* argv[]) {
	QApplication app(argc, argv);

	MainWindow window;
	window.setWindowTitle("MISO");
	window.show();

	return app.exec();
}
