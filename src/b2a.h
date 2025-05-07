#pragma once

#include <vector>

enum class b2a_token {
    IDENTIFIER,
    KEYWORD,
    OPERATOR,
    NUMBER,
    STRING,
    SEPERATOR,
    LINE_END,
    EXPR_END,
    UNKNOWN,
};

static inline const char* b2a_token_tostr(b2a_token token) {
    switch (token) {
        case b2a_token::IDENTIFIER: return "IDENTIFIER";
        case b2a_token::KEYWORD:    return "KEYWORD";
        case b2a_token::OPERATOR:   return "OPERATOR";
        case b2a_token::NUMBER:     return "NUMBER";
        case b2a_token::STRING:     return "STRING";
        case b2a_token::SEPERATOR:  return "SEPERATOR";
        case b2a_token::LINE_END:   return "LINE_END";
        case b2a_token::EXPR_END:   return "EXPR_END";
        case b2a_token::UNKNOWN:    return "UNKNOWN";
        default:                    return "??????";
    }
}

enum class b2a_instruction_type {
    VAR_CREATE,
    VAR_ASSIGN,
    FUNC_CALL,
};


struct b2a_instr_var_create {
    std::string name;
    std::string scope;
};

struct b2a_instr_var_assign {
    std::string name;
    std::string value;
};

struct b2a_instruction {
    b2a_instruction_type type;
    void* data;
};


std::vector<std::pair<b2a_token, std::string>> b2a_lexer(std::ifstream* input);

