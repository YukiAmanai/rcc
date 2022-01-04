(
    Node {
        lhs: Some(
            Node {
                lhs: Some(
                    Node {
                        lhs: None,
                        rhs: None,
                        val: Some(
                            12,
                        ),
                        operator: None,
                    },
                ),
                rhs: Some(
                    Node {
                        lhs: None,
                        rhs: None,
                        val: Some(
                            2,
                        ),
                        operator: None,
                    },
                ),
                val: None,
                operator: Some(
                    '*',
                ),
            },
        ),
        rhs: Some(
            Node {
                lhs: Some(
                    Node {
                        lhs: None,
                        rhs: None,
                        val: Some(
                            2,
                        ),
                        operator: None,
                    },
                ),
                rhs: Some(
                    Node {
                        lhs: None,
                        rhs: None,
                        val: Some(
                            3,
                        ),
                        operator: None,
                    },
                ),
                val: None,
                operator: Some(
                    '*',
                ),
            },
        ),
        val: None,
        operator: Some(
            '+',
        ),
    },
    [
        Token {
            val: None,
            op: Some(
                '+',
            ),
        },
        Token {
            val: Some(
                2,
            ),
            op: None,
        },
        Token {
            val: None,
            op: Some(
                '*',
            ),
        },
        Token {
            val: Some(
                3,
            ),
            op: None,
        },
    ],
)
.intel_syntax noprefix
.global main
main:
  pop rax  ret""
