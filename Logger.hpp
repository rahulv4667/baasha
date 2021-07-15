#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <memory>

namespace Baasha {

    enum class LogLevel {
        WARNING,
        ERROR,
        CRASH
    };

    class Logger {
        private:

            void printLogLevel(LogLevel level) {
                switch(level) {
                    case LogLevel::WARNING: std::cerr<<"[WARN] ";       return;
                    case LogLevel::ERROR:   std::cerr<<"[ERROR] ";      return;
                    case LogLevel::CRASH:   std::cerr<<"[CRASH] ";      return;
                    default:                std::cerr<<"[UNKNOWN] ";    return;
                }
            }


        private:
            // static inline std::unique_ptr<>

        public:
            Logger() {}
            virtual ~Logger() {}
        
            Logger(const Logger&) = delete;
            Logger& operator=(const Logger) = delete;

            static std::shared_ptr<Logger> getInstance() {
                static std::shared_ptr<Logger> logger;
                if(logger.get() == nullptr) {
                    logger = std::make_unique<Logger>();
                }
                return logger;
            } 

            void logMessage(LogLevel level, std::string message) {
                printLogLevel(level);
                std::cerr<<message<<std::endl;
                if(level == LogLevel::CRASH) exit(EXIT_FAILURE);
            }

            void logMessage(LogLevel level, const char* format, ...) {
                va_list args;
                printLogLevel(level);
                va_start(args, format);
                vfprintf(stderr, format, args);
                va_end(args);
                fprintf(stderr, "\n");
                if(level == LogLevel::CRASH) exit(EXIT_FAILURE);
            }
            
    };

}