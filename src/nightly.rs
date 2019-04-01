
use syntax::source_map::Span;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use rustc_plugin::Registry;
use syntax::ast::ExprKind;
use syntax::ast::LitKind;
use syntax::ast::Mutability;
use syntax::ast::LitIntType;
use syntax::ast::UintTy;
use syntax::ast::BlockCheckMode;
use syntax::ast::Block;
use syntax::ptr::P;
use syntax::parse::token::Token;
use syntax::ast;
use syntax::ast::IntTy;


#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	reg.register_macro("concat_bytes", cstr);
}

#[doc(hidden)]
pub fn cstr(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
	let mut parser = cx.new_parser_from_tts(args);
	
	let mut args_len = args.len();
	let mut array_expr = Vec::with_capacity(args_len / 2);
	if args_len > 0 {
		match parser.parse_expr() {
			Ok(a) => array_expr.push(a),
			Err(_e) => {
				cx.span_err(parser.span, "incorrect data, was expected: &[u8], str, u8, i8");
				return DummyResult::any(sp);
			}
		}
		let mut count_elements = 1;
		while parser.eat(&Token::Comma) {
			args_len -= 1;
			//del comma
			
			match parser.parse_expr() {
				Ok(a) => {
					count_elements += 1;
					array_expr.push(a);
				},
				Err(_e) => {
					cx.span_err(parser.span, "incorrect data, was expected: &[u8], str, u8, i8");
					return DummyResult::any(sp);
				},
			}
		}
		if count_elements != args_len {
			cx.span_err(parser.span, "It was expected ',' or closing of a macro.");
			return DummyResult::any(sp);
		}
	}

	let mut r_array = Vec::with_capacity(args_len * 5);
	'looper: for unk in array_expr.into_iter() {
		match unk.node {
			ExprKind::Lit(ref l) => {
				match l.node {
					LitKind::Str(ref array, _) => {
						let s_array = array.as_str();
						let array = s_array.as_bytes();
						
						for a in array.into_iter() {
							r_array.push(
								cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
							);
						}
					},
					LitKind::ByteStr(ref array) => {	
						let array = array.as_slice();					
						for a in array.into_iter() {
							r_array.push(
								cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
							);
						}
					},
					
					/*LitKind::Int(0, LitIntType::Unsuffixed) => {
						if unk_index+1 == args_len {
							break 'looper;
						}else {
							cx.span_err(l.span, "trailing byte detected");
							return DummyResult::any(sp);
						}
					},*/
					
					LitKind::Int(ref a, LitIntType::Unsigned(UintTy::U8)) 
					| LitKind::Int(ref a, LitIntType::Signed(IntTy::I8))
					=> {
						r_array.push(
							cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
						);
					},
					
					LitKind::Byte(ref a) => {
						r_array.push(
							cx.expr_lit(sp, LitKind::Int(*a as u128, LitIntType::Unsigned(UintTy::U8)))
						);
					},
					_ => {
						cx.span_err(l.span, "incorrect data, was expected: &[u8], str, u8, i8");
						return DummyResult::any(sp);
					}
				}
			},
			_ => {
				cx.span_err(unk.span, "incorrect data, was expected: &[u8], str, u8, i8");
				return DummyResult::any(sp);
			}
		}
		drop(unk);
	}

	//END ARRAY
	
	let result = cx.expr(
		sp, 
		ExprKind::AddrOf(Mutability::Immutable,
			cx.expr(sp, ExprKind::Array(r_array)) //[u8]
			//ARRAY EXPR u8 -> EXPR [U8]
		) 
	);// & [u8]
	
	MacEager::expr({
		let block = P(Block {
			stmts: { //{block}
				let mut r = Vec::with_capacity(1);
				r.push(
					cx.stmt_expr(result) //ADD EXPR TO BLOCK
				);
				r
			},
			id: ast::DUMMY_NODE_ID,
			rules: BlockCheckMode::Default, //<-- UNSAFE
			span: sp, 
			
			//recovered: false,
			//FIX!, UPDATE RUST:((
		});
		//cx.expr(sp, Expr::E) //RESULT EXPR
		cx.expr_block(block)
	})// { &[u8] }
}


