#include <iostream>
#include <fstream>
#include <vector>

#include "b2a.h"

using std::vector;
using std::pair;
using std::ifstream;
using std::string;

pair<b2a_token, string> b2a_next_token(string line, size_t& pos) {
    // skip whitespace
    while (pos < line.size() && std::isspace(line[pos])) {
        ++pos;
    }

    // if end of line is reached, return
    if (pos == line.size()) return {b2a_token::LINE_END, "\n"};
    
    // identifiers
    if (std::isalpha(line[pos])) {
        size_t start = pos;
        // read until a non-alphenumeric character is found
        // (exception is $ as that is used to determine a string var)
        while (pos < line.size() && (std::isalnum(line[pos]) || line[pos] == '$')) {
            ++pos;
        }
        // cut it out
        string value = line.substr(start, pos - start);
        return {b2a_token::IDENTIFIER, value};
    }
    // numbers
    if (std::isdigit(line[pos])) {
        size_t start = pos;
        while (pos < line.size() && std::isdigit(line[pos])) {
            ++pos;
        }
        return {b2a_token::NUMBER, line.substr(start, pos - start)};
    }

    //  operators
    if (line[pos] == '=' || line[pos] == '+' || line[pos] == '-' || line[pos] == '*' || line[pos] == '/') {
        ++pos;
        return {b2a_token::OPERATOR, std::string(1, line[pos-1])};
    }

    if (line[pos] == '"') {
        string str;
        bool escaped = false;
        while (pos < line.size()) {
            pos++;
            if (line[pos] == '"' && !escaped) {
                ++pos;
                break;
            } else if (line[pos] == '\\') {
                escaped = true;
            } else {
                str += line[pos];
                escaped = false;
            }
        }
        return {b2a_token::STRING, str};
    }

    if (line[pos] == ',') {
        ++pos;
        return {b2a_token::SEPERATOR, ","};
    }

    if (line[pos] == ';') {
        ++pos;
        return {b2a_token::EXPR_END, ";"};
    }

    ++pos;
    return {b2a_token::UNKNOWN, string(1, line[pos-1])};
}

vector<pair<b2a_token, string>> b2a_lexer(ifstream* input) {
    vector<pair<b2a_token, string>> vec = {};

    string line;
    while (std::getline(*input, line)) {
        size_t pos = 0;
        pair<b2a_token, string> last_token;
        while( (last_token = b2a_next_token(line, pos)).first != b2a_token::LINE_END ) {
            // turn LINE_ENDs into EXPR_ENDs
            if (last_token.first == b2a_token::LINE_END) {
                vec.push_back({b2a_token::EXPR_END, last_token.second});
            } else {
                vec.push_back(last_token);
            }
        }

        // line finished
        vec.push_back({b2a_token::EXPR_END, "\n"});
    }

    return vec;
}
