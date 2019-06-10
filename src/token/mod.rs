
use std::collections::HashMap;


pub struct Token {
	Type: &'static str,
	Literal: String,
	Row: i32,
	Column: i32,
}


const Illegal: &str   = "Illegal";
const EOF: &str       = "EOF";

const Ident: &str     = "Ident";     // $add, $foobar, $x, $y, ...
const ProcIdent: &str = "ProcIdent"; // add, FuncName, ...
const Int: &str       = "Int";       // 1343456
const Int16: &str     = "Int16";     // 0xA0, 0xfff, ...
const Float: &str     = "Float";     // 1.1, 1e-3, 1e+3, ...
const String: &str    = "String";    // "node.attr", ...
const Flag: &str      = "Flag";      // -size, -s, ...
const True: &str      = "True";
const On: &str        = "On";
const False: &str     = "False";
const Off: &str       = "Off";

const Assign: &str   = "=";
const Plus: &str     = "+";
const Minus: &str    = "-";
const Slash: &str    = "/";
const Asterisk: &str = "*";
const Bang: &str     = "!";
const Mod: &str      = "%";
const Dot: &str      = ".";
const Hat: &str      = "^";

const Lt: &str   = "<";
const Gt: &str   = ">";
const LtEq: &str = "<=";
const GtEq: &str = ">=";

const Question: &str = "?";
const Coron: &str    = ":";

const Eq: &str        = "==";
const PAssign: &str   = "+=";
const MAssign: &str   = "-=";
const SAssign: &str   = "/=";
const AAssign: &str   = "*=";
const NotEq: &str     = "!=";
const And: &str       = "&&";
const Or: &str        = "||";
const Increment: &str = "++";
const Decrement: &str = "--";

const Comma: &str      = ",";
const Semicolon: &str  = ";";
const BackQuotes: &str = "`";

const Lparen: &str   = "(";
const Rparen: &str   = ")";
const Lbrace: &str   = "{";
const Rbrace: &str   = "}";
const Lbracket: &str = "[";
const Rbracket: &str = "]";
const Ltensor: &str  = "<<";
const Rtensor: &str  = ">>";

const Global: &str    = "Global";
const Proc: &str      = "Proc";
const StringDec: &str = "StringDec";
const IntDec: &str    = "IntDec";
const FloatDec: &str  = "FloatDec";
const VectorDec: &str = "VectorDec";
const MatrixDec: &str = "MatrixDec";
const If: &str        = "If";
const While: &str     = "While";
const Do: &str        = "Do";
const Switch: &str    = "Switch";
const Case: &str      = "Case";
const Default: &str   = "Default";
const Break: &str     = "Break";
const Continue: &str  = "Continue";
const For: &str       = "For";
const In: &str        = "In";
const Else: &str      = "Else";
const Return: &str    = "Return";


fn get_mapping() -> HashMap<&'static str, &'static str> {

	let keywords = HashMap::new();
	keywords.insert("global",   Global);
	keywords.insert("proc",     Proc);
	keywords.insert("string",   StringDec);
	keywords.insert("int",      IntDec);
	keywords.insert("float",    FloatDec);
	keywords.insert("vector",   VectorDec);
	keywords.insert("matrix",   MatrixDec);
	keywords.insert("true",     True);
	keywords.insert("false",    False);
	keywords.insert("on",       On);
	keywords.insert("off",      Off);
	keywords.insert("if",       If);
	keywords.insert("while",    While);
	keywords.insert("do",       Do);
	keywords.insert("switch",   Switch);
	keywords.insert("case",     Case);
	keywords.insert("default",  Default);
	keywords.insert("break",    Break);
	keywords.insert("continue", Continue);
	keywords.insert("for",      For);
	keywords.insert("in",       In);
	keywords.insert("else",     Else);
	keywords.insert("return",   Return);
	keywords
}


// LookupIdent ...
fn LookupIdent(ident: &str) -> &str {
	match get_mapping().get(ident) {
		Some(val) => val,
		None => ProcIdent,
	}
}
