#include <iostream>
#include <fstream>
#include <vector>
#include <regex>

#include "b2a.h"

using std::cout;
using std::cerr;
using std::string;
using std::ifstream;
using std::ofstream;

int main(int argc, char** argv) {
    ifstream input;
    ofstream output = ofstream("a.s");
    
    for(int i = 1; i < argc; i++) {
        string arg = argv[i];
        if (arg.rfind("-i=") == 0) {
            input = ifstream(arg.substr(3));
        } else if (arg.rfind("-i") == 0 && arg.length() == 2) {
            if (i+1 < argc) {
                input = ifstream(argv[i+1]);
            } else {
                cerr << argv[0] << ": Expected a value for -i\n";
                return 1;
            }
        } else if (arg.rfind("-i") == 0 && arg.length() > 2) {
            input = ifstream(arg.substr(2));
        } else if (arg.rfind("-o=") == 0) {
            output = ofstream(arg.substr(3));
        } else if (arg.rfind("-o") == 0 && arg.length() == 2) {
            if (i+1 < argc) {
                output = ofstream(argv[i+1]);
            } else {
                cerr << argv[0] << ": Expected a value for -o\n";
                return 1;
            }
        } else if (arg.rfind("-o") == 0 && arg.length() > 2) {
            output = ofstream(arg.substr(2));
        }
    }

    /*if (!input || !input.is_open())
        cerr << argv[0] << ": error opening input file!\n";
        return 1;
    if (!output || !output.is_open()) {
        cerr << argv[0] << ": error opening output file!\n";
        return 1;
    }*/

    std::vector<std::pair<b2a_token, std::string>> tokens = b2a_lexer(&input);
    for(int i = 0; i < tokens.size(); i++) {
        auto token = tokens[i];
        cout << "t[" << i << "] = {" << b2a_token_tostr(token.first) << ", '" << std::regex_replace(token.second, std::regex("\n"), "\\n") << "'}\n";
    }
    //b2a_transpile(tokens, output);

    input.close();
    output.close();

    return 0;
}

