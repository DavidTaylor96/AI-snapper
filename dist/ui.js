"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.printHeader = printHeader;
exports.printStatus = printStatus;
exports.printSuccess = printSuccess;
exports.printError = printError;
exports.printAnalysisResult = printAnalysisResult;
exports.createSpinner = createSpinner;
exports.updateSpinner = updateSpinner;
exports.stopSpinner = stopSpinner;
const chalk_1 = __importDefault(require("chalk"));
const ora_1 = __importDefault(require("ora"));
function printHeader() {
    console.clear();
    console.log(chalk_1.default.cyan('🤖 AI Screenshot Analyzer - Node.js Edition'));
    console.log(chalk_1.default.cyan('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━'));
}
function printStatus(message) {
    console.log(chalk_1.default.yellow(message));
}
function printSuccess(message) {
    console.log(chalk_1.default.green(message));
}
function printError(message) {
    console.log(chalk_1.default.red(message));
}
function printAnalysisResult(analysis) {
    // Simple, clean formatting for the analysis result
    const lines = analysis.split('\n');
    let inCodeBlock = false;
    for (const line of lines) {
        if (line.trim().startsWith('┌─ CODE SOLUTION')) {
            // Code block header - make it bright and noticeable
            console.log(chalk_1.default.green(line));
        }
        else if (line.trim().startsWith('└─')) {
            // Code block footer
            console.log(chalk_1.default.green(line));
        }
        else if (line.trim().startsWith('```')) {
            if (!inCodeBlock) {
                // Starting code block
                console.log(chalk_1.default.yellow(line));
                inCodeBlock = true;
            }
            else {
                // Ending code block
                console.log(chalk_1.default.yellow(line));
                inCodeBlock = false;
            }
        }
        else if (inCodeBlock) {
            // Code content - bright white on black for visibility
            console.log(chalk_1.default.bgBlack.white(line));
        }
        else if (line.trim().startsWith('─')) {
            // Separator lines
            console.log(chalk_1.default.blue(line));
        }
        else if (line.includes('🤖 ChatGPT Analysis')) {
            // Header
            console.log(chalk_1.default.cyan(line));
        }
        else {
            // Regular text
            console.log(chalk_1.default.white(line));
        }
    }
    // Add copy instruction
    console.log(chalk_1.default.gray('\n💡 Tip: Select and copy code between the ``` markers'));
}
function createSpinner(message) {
    return (0, ora_1.default)(message).start();
}
function updateSpinner(spinner, message) {
    spinner.text = message;
}
function stopSpinner(spinner, success = true) {
    if (success) {
        spinner.succeed();
    }
    else {
        spinner.fail();
    }
}
//# sourceMappingURL=ui.js.map