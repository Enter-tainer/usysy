const PREC = {
  PAREN_DECLARATOR: -10,
  ASSIGNMENT: -1,
  CONDITIONAL: -2,
  DEFAULT: 0,
  LOGICAL_OR: 1,
  LOGICAL_AND: 2,
  INCLUSIVE_OR: 3,
  EXCLUSIVE_OR: 4,
  BITWISE_AND: 5,
  EQUAL: 6,
  RELATIONAL: 7,
  SIZEOF: 8,
  SHIFT: 9,
  ADD: 10,
  MULTIPLY: 11,
  CAST: 12,
  UNARY: 13,
  CALL: 14,
  FIELD: 15,
  SUBSCRIPT: 16
};

module.exports = grammar({
  name: 'sysy',

  extras: $ => [
    /\s|\\\r?\n/,
    $.comment,
  ],

  conflicts: $ => [
    [$.declarator, $.function_definition],
    [$.if_statement, $.call_expression],
  ],

  word: $ => $.identifier,

  rules: {
    translation_unit: $ => repeat($._top_level_item),

    _top_level_item: $ => choice(
      $.function_definition,
      $.declaration,
    ),

    // Main Grammar

    function_definition: $ => seq(
      field('return_type', $.primitive_type),
      field('name', $.identifier),
      field('param', $.parameter_list),
      field('body', $.compound_statement)
    ),

    declaration: $ => seq(
      field('const', optional($.type_qualifier)),
      field('type', $.primitive_type),
      commaSep1(
        $.declarator
      ),
      ';'
    ),
    declarator_array_dimension: $ => repeat1(seq(
      '[',
      $._expression,
      ']',
    )),
    declarator: $ => seq(
      field('name', $.identifier),
      field('dimension', optional($.declarator_array_dimension)),
      optional(
        seq(
          '=',
          field('init', $.init_value)
        )
      )
    ),
    init_value: $ => $._init_value,
    empty_init_list: $ => seq('{', '}'),
    init_list: $ => seq(
      '{',
      commaSep1($._init_value),
      '}'
    ),
    _init_value: $ => choice(
      $._expression,
      $.empty_init_list,
      $.init_list,
    ),

    parameter_list: $ => choice(
      seq('(', ')'),
      seq(
      '(',
        $._parameter_list,
      ')'
    )),
    _parameter_list: $ => commaSep1(
      $.parameter,
    ),
    parameter_array: $ => seq(
      $.empty_array,
      repeat($.parameter_array_dimension)
    ),
    parameter: $ => seq(
      field('type', $.primitive_type),
      field('name', $.identifier),
      field('array', optional(
        $.parameter_array
      ))
    ),
    empty_array: $ => token(seq('[', ']')),
    parameter_array_dimension: $ => seq(
      '[',
      $._expression,
      ']'
    ),


    type_qualifier: $ => choice(
      'const',
    ),

    primitive_type: $ => token(choice(
      'int',
      'float',
      'void',
    )),

    compound_statement: $ => seq(
      '{',
      repeat($._statement),
      '}'
    ),
    
    _statement: $ => choice(
      seq($.assignment, ';'),
      $.expression_statement,
      $.compound_statement,
      $.if_statement,
      $.while_statement,
      $.break_statement,
      $.continue_statement,
      $.return_statement,
      $.declaration,
    ),

    if_statement: $ => prec.right(seq(
      'if',
      field('condition', $.parenthesized_expression),
      field('consequence', $._statement),
      optional(seq(
        'else',
        field('alternative', $._statement)
      ))
    )),

    while_statement: $ => seq(
      'while',
      field('condition', $.parenthesized_expression),
      field('body', $._statement)
    ),

    return_statement: $ => seq(
      'return',
      field('return_value', $._expression),
      ';'
    ),

    break_statement: $ => seq(
      'break', ';'
    ),

    continue_statement: $ => seq(
      'continue', ';'
    ),

    expression_statement: $ => seq(
      optional(
        $._expression,
      ),
      ';'
    ),

    // Expressions

    _expression: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.subscript_expression,
      $.call_expression,
      $.identifier,
      $._number_literal,
      $.parenthesized_expression
    ),

    _assignment_left_expression: $ => choice(
      $.identifier,
      $.subscript_expression,
    ),

    assignment: $ => prec.right(PREC.ASSIGNMENT, seq(
      field('left', $._assignment_left_expression),
      '=',
      field('right', $._expression)
    )),

    unary_expression: $ => prec.left(PREC.UNARY, seq(
      field('operator', choice('!', '-', '+')),
      field('argument', $._expression)
    )),

    binary_expression: $ => {
      const table = [
        ['+', PREC.ADD],
        ['-', PREC.ADD],
        ['*', PREC.MULTIPLY],
        ['/', PREC.MULTIPLY],
        ['%', PREC.MULTIPLY],
        ['||', PREC.LOGICAL_OR],
        ['&&', PREC.LOGICAL_AND],
        ['==', PREC.EQUAL],
        ['!=', PREC.EQUAL],
        ['>', PREC.RELATIONAL],
        ['>=', PREC.RELATIONAL],
        ['<=', PREC.RELATIONAL],
        ['<', PREC.RELATIONAL],
      ];

      return choice(...table.map(([operator, precedence]) => {
        return prec.left(precedence, seq(
          field('left', $._expression),
          field('operator', operator),
          field('right', $._expression)
        ))
      }));
    },

    subscript_expression: $ => prec(PREC.SUBSCRIPT, seq(
      field('argument', $._expression),
      '[',
      field('index', $._expression),
      ']'
    )),

    call_expression: $ => prec(PREC.CALL, seq(
      field('function', $._expression),
      field('arguments', $.argument_list)
    )),

    argument_list: $ => seq('(', commaSep($._expression), ')'),

    parenthesized_expression: $ => seq(
      '(',
      $._expression,
      ')'
    ),

    _number_literal: $ => choice(
      $.float_literal,
      $.int_literal
    ),

    float_literal: $ => {
      const hex = /[0-9a-fA-F]/;
      const decimal = /[0-9]/;
      const hexDigits = repeat1(hex);
      const decimalDigits = repeat1(decimal);
      const exponentPart = seq(choice('e', 'E'), optional(choice('+', '-')), decimalDigits);
      const binaryExponentPart = seq(choice('P', 'p'), optional(choice('+', '-')), decimalDigits);
      const decimalFloating = choice(
        seq(
          choice(
            seq(
              optional(
                decimalDigits
              ),
              '.',
              decimalDigits
            ),
            seq(decimalDigits, '.')
          ),
          optional(exponentPart)
        ),
        seq(
          decimalDigits,
          exponentPart
        )
      );
      const hexFloating = seq(
        choice('0x', '0X'),
        choice(
          seq(
            hexDigits,
            binaryExponentPart
          ),
          seq(
            choice(
              seq(
                optional(hexDigits),
                '.',
                hexDigits
              ),
              seq(hexDigits, '.')
            ),
            binaryExponentPart
          )
        )
      )
      return token(
        choice(
          decimalFloating,
          hexFloating,
        )
      )
    },

    int_literal: $ => {
      const hex = /[0-9a-fA-F]/;
      const decimal = /[0-9]/;
      const octo = /[0-7]/;
      const hexDigits = repeat1(hex);
      return token(
        choice(
          '0',
          seq(
            '0',
            repeat(octo),
          ),
          seq(
            /[1-9]/,
            repeat(decimal),
          ),
          seq(
            choice('0x', '0X'),
            hexDigits,
          ),
        )
      )
    },

    identifier: $ => /[a-zA-Z_]\w*/,

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: $ => token(choice(
      seq('//', /(\\(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/'
      )
    )),
  },

});

module.exports.PREC = PREC

function commaSep(rule) {
  return optional(commaSep1(rule))
}

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)))
}

function commaSepTrailing(recurSymbol, rule) {
  return choice(rule, seq(recurSymbol, ',', rule))
}
