	.text
	.file	"for_iter_1.ea46651434c0a444-cgu.0"
	.section	".text._ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hb6bb605b6728e365E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hb6bb605b6728e365E,@function
_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hb6bb605b6728e365E:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rsi, 8(%rsp)
	movq	%rdi, 16(%rsp)
	movq	%rdi, 24(%rsp)
	movq	8(%rsp), %rax
	movq	24(%rax), %rdi
	movq	8(%rax), %rsi
	callq	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h4f8c88dcc015f813E
	movq	%rax, 32(%rsp)
	movq	24(%rsp), %rax
	movq	16(%rsp), %rcx
	movq	32(%rsp), %rdx
	movq	32(%rsp), %rsi
	movq	%rsi, 48(%rsp)
	movq	$1, 40(%rsp)
	movq	%rdx, (%rcx)
	movq	40(%rsp), %rsi
	movq	48(%rsp), %rdx
	movq	%rsi, 8(%rcx)
	movq	%rdx, 16(%rcx)
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hb6bb605b6728e365E, .Lfunc_end0-_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hb6bb605b6728e365E
	.cfi_endproc

	.section	".text._ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h1f5047ef96ba56e1E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h1f5047ef96ba56e1E,@function
_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h1f5047ef96ba56e1E:
	.cfi_startproc
	movq	%rdi, -16(%rsp)
	movq	(%rdi), %rax
	movq	%rax, -8(%rsp)
	movq	-16(%rsp), %rax
	movq	(%rax), %rcx
	addq	$1, %rcx
	movq	%rcx, (%rax)
	movq	-8(%rsp), %rax
	retq
.Lfunc_end1:
	.size	_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h1f5047ef96ba56e1E, .Lfunc_end1-_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h1f5047ef96ba56e1E
	.cfi_endproc

	.section	".text._ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h47d0ffd459aad920E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h47d0ffd459aad920E,@function
_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h47d0ffd459aad920E:
	.cfi_startproc
	movq	%rdi, -16(%rsp)
	movq	(%rdi), %rax
	movq	%rax, -8(%rsp)
	movq	-16(%rsp), %rax
	movq	(%rax), %rcx
	addq	$4, %rcx
	movq	%rcx, (%rax)
	movq	-8(%rsp), %rax
	retq
.Lfunc_end2:
	.size	_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h47d0ffd459aad920E, .Lfunc_end2-_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h47d0ffd459aad920E
	.cfi_endproc

	.section	".text._ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h67b7713e10dbee24E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h67b7713e10dbee24E,@function
_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h67b7713e10dbee24E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	subq	$200, %rsp
	.cfi_def_cfa_offset 208
	movq	%rdi, 24(%rsp)
	movq	%rdi, 32(%rsp)
	movq	%rsi, 48(%rsp)
	movq	%rdx, 56(%rsp)
	movb	$1, 151(%rsp)
.Ltmp0:
	leaq	48(%rsp), %rdi
	callq	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2259e18d6fa02539E
.Ltmp1:
	movl	%eax, 44(%rsp)
	jmp	.LBB3_3
.LBB3_1:
	testb	$1, 151(%rsp)
	jne	.LBB3_20
	jmp	.LBB3_19
.LBB3_2:
.Ltmp2:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 176(%rsp)
	movl	%eax, 184(%rsp)
	jmp	.LBB3_1
.LBB3_3:
	movl	44(%rsp), %eax
	movl	%eax, 92(%rsp)
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpl	$1114112, 92(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB3_5
	movq	24(%rsp), %rax
	movq	$0, (%rax)
	movl	$4, %ecx
	movq	%rcx, 8(%rax)
	movq	$0, 16(%rax)
	jmp	.LBB3_6
.LBB3_5:
	movl	92(%rsp), %eax
	movl	%eax, 20(%rsp)
.Ltmp3:
	leaq	96(%rsp), %rdi
	leaq	48(%rsp), %rsi
	callq	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h42b2953809891607E
.Ltmp4:
	jmp	.LBB3_9
.LBB3_6:
	movq	32(%rsp), %rax
	addq	$200, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB3_7:
	.cfi_def_cfa_offset 208
	jmp	.LBB3_1
.LBB3_8:
.Ltmp11:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 176(%rsp)
	movl	%eax, 184(%rsp)
	jmp	.LBB3_7
.LBB3_9:
	movq	96(%rsp), %rax
	incq	%rax
	movq	$-1, %rcx
	cmoveq	%rcx, %rax
	movq	%rax, 192(%rsp)
	movq	192(%rsp), %rsi
.Ltmp5:
	movl	$4, %edi
	callq	_ZN4core3cmp6max_by17h9983db75b730376cE
.Ltmp6:
	movq	%rax, 8(%rsp)
	jmp	.LBB3_10
.LBB3_10:
.Ltmp7:
	movq	8(%rsp), %rsi
	leaq	152(%rsp), %rdi
	xorl	%edx, %edx
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h015f25f831395c3cE
.Ltmp8:
	jmp	.LBB3_11
.LBB3_11:
	cmpq	$0, 152(%rsp)
	jne	.LBB3_13
	movl	20(%rsp), %ecx
	movq	160(%rsp), %rdx
	movq	168(%rsp), %rax
	movq	%rdx, 120(%rsp)
	movq	%rax, 128(%rsp)
	movq	$0, 136(%rsp)
	movq	128(%rsp), %rax
	movl	%ecx, (%rax)
	movq	$1, 136(%rsp)
	movq	136(%rsp), %rax
	movq	%rax, 80(%rsp)
	movups	120(%rsp), %xmm0
	movaps	%xmm0, 64(%rsp)
	movb	$0, 151(%rsp)
	movq	48(%rsp), %rsi
	movq	56(%rsp), %rdx
.Ltmp12:
	leaq	64(%rsp), %rdi
	callq	_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17ha52a72e1e0acc633E
.Ltmp13:
	jmp	.LBB3_16
.LBB3_13:
	movq	160(%rsp), %rdi
	movq	168(%rsp), %rsi
.Ltmp9:
	movq	_ZN5alloc7raw_vec12handle_error17hd343a2caeadfaacfE@GOTPCREL(%rip), %rax
	callq	*%rax
.Ltmp10:
	jmp	.LBB3_18
.LBB3_14:
.Ltmp15:
	leaq	64(%rsp), %rdi
	callq	_ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE
.Ltmp16:
	jmp	.LBB3_1
.LBB3_15:
.Ltmp14:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 176(%rsp)
	movl	%eax, 184(%rsp)
	jmp	.LBB3_14
.LBB3_16:
	movq	24(%rsp), %rax
	movq	64(%rsp), %rcx
	movq	%rcx, (%rax)
	movq	72(%rsp), %rcx
	movq	%rcx, 8(%rax)
	movq	80(%rsp), %rcx
	movq	%rcx, 16(%rax)
	jmp	.LBB3_6
.LBB3_17:
.Ltmp17:
	movq	_ZN4core9panicking16panic_in_cleanup17he962919aa9bfab8cE@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB3_18:
	ud2
.LBB3_19:
	movq	176(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB3_20:
	jmp	.LBB3_19
.Lfunc_end3:
	.size	_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h67b7713e10dbee24E, .Lfunc_end3-_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h67b7713e10dbee24E
	.cfi_endproc
	.section	".gcc_except_table._ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h67b7713e10dbee24E","a",@progbits
	.p2align	2, 0x0
GCC_except_table3:
.Lexception0:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.uleb128 .Ltmp8-.Ltmp3
	.uleb128 .Ltmp11-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp12-.Lfunc_begin0
	.uleb128 .Ltmp13-.Ltmp12
	.uleb128 .Ltmp14-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp9-.Lfunc_begin0
	.uleb128 .Ltmp10-.Ltmp9
	.uleb128 .Ltmp11-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp15-.Lfunc_begin0
	.uleb128 .Ltmp16-.Ltmp15
	.uleb128 .Ltmp17-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp16-.Lfunc_begin0
	.uleb128 .Lfunc_end3-.Ltmp16
	.byte	0
	.byte	0
.Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	".text._ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe6586ba72b043bE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe6586ba72b043bE,@function
_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe6586ba72b043bE:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	(%rdi), %rax
	movq	(%rax), %rax
	movq	%rax, (%rsp)
	movq	(%rdi), %rax
	movq	16(%rax), %rax
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rax
	movq	%rax, 32(%rsp)
	movq	(%rsp), %rcx
	movq	32(%rsp), %rax
	movq	%rcx, 24(%rsp)
	movq	%rax, 16(%rsp)
	leaq	16(%rsp), %rdi
	callq	_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17hf0d179b9f1cf3ebaE
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe6586ba72b043bE, .Lfunc_end4-_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe6586ba72b043bE
	.cfi_endproc

	.section	.text._ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h90071b413f84eb44E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h90071b413f84eb44E,@function
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h90071b413f84eb44E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ops8function6FnOnce9call_once17hc9488438d58459e6E
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h90071b413f84eb44E, .Lfunc_end5-_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h90071b413f84eb44E
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17h54ad9d19b72f2e73E,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h54ad9d19b72f2e73E
	.globl	_ZN3std2rt10lang_start17h54ad9d19b72f2e73E
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start17h54ad9d19b72f2e73E,@function
_ZN3std2rt10lang_start17h54ad9d19b72f2e73E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movl	%ecx, %eax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, 16(%rsp)
	leaq	16(%rsp), %rdi
	leaq	.L__unnamed_1(%rip), %rsi
	movzbl	%al, %r8d
	callq	*_ZN3std2rt19lang_start_internal17hbab5e242a3e5d4bdE@GOTPCREL(%rip)
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end6:
	.size	_ZN3std2rt10lang_start17h54ad9d19b72f2e73E, .Lfunc_end6-_ZN3std2rt10lang_start17h54ad9d19b72f2e73E
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h90071b413f84eb44E
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcf46670d88e87af2E
	movb	%al, 7(%rsp)
	movzbl	7(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end7:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E, .Lfunc_end7-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E
	.cfi_endproc

	.section	".text._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h46b5eb0011ded131E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h46b5eb0011ded131E,@function
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h46b5eb0011ded131E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h604a1f0d2d4c45c1E
	andb	$1, %al
	movzbl	%al, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end8:
	.size	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h46b5eb0011ded131E, .Lfunc_end8-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h46b5eb0011ded131E
	.cfi_endproc

	.section	".text._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8212252bd2e22ba4E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8212252bd2e22ba4E,@function
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8212252bd2e22ba4E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h28b1788d2644cd66E
	andb	$1, %al
	movzbl	%al, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8212252bd2e22ba4E, .Lfunc_end9-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8212252bd2e22ba4E
	.cfi_endproc

	.section	".text._ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb71c0bc844d5fd4bE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb71c0bc844d5fd4bE,@function
_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb71c0bc844d5fd4bE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	*_ZN43_$LT$char$u20$as$u20$core..fmt..Display$GT$3fmt17hc1c6eced204b85d2E@GOTPCREL(%rip)
	andb	$1, %al
	movzbl	%al, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb71c0bc844d5fd4bE, .Lfunc_end10-_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb71c0bc844d5fd4bE
	.cfi_endproc

	.section	.text._ZN4core10intrinsics11write_bytes18precondition_check17h0e3014efe4c8c704E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core10intrinsics11write_bytes18precondition_check17h0e3014efe4c8c704E,@function
_ZN4core10intrinsics11write_bytes18precondition_check17h0e3014efe4c8c704E:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rdi, (%rsp)
	movq	%rsi, 8(%rsp)
	cmpq	$0, %rdi
	jne	.LBB11_2
	jmp	.LBB11_3
.LBB11_2:
	movq	8(%rsp), %rcx
	movq	%rcx, %rax
	shrq	%rax
	movabsq	$6148914691236517205, %rdx
	andq	%rdx, %rax
	subq	%rax, %rcx
	movabsq	$3689348814741910323, %rdx
	movq	%rcx, %rax
	andq	%rdx, %rax
	shrq	$2, %rcx
	andq	%rdx, %rcx
	addq	%rcx, %rax
	movq	%rax, %rcx
	shrq	$4, %rcx
	addq	%rcx, %rax
	movabsq	$1085102592571150095, %rcx
	andq	%rcx, %rax
	movabsq	$72340172838076673, %rcx
	imulq	%rcx, %rax
	shrq	$56, %rax
	movl	%eax, 68(%rsp)
	cmpl	$1, 68(%rsp)
	je	.LBB11_4
	jmp	.LBB11_5
.LBB11_3:
	leaq	.L__unnamed_2(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$111, %esi
	callq	*%rax
.LBB11_4:
	movq	(%rsp), %rax
	movq	8(%rsp), %rcx
	subq	$1, %rcx
	andq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB11_6
	jmp	.LBB11_3
.LBB11_5:
	leaq	.L__unnamed_3(%rip), %rax
	movq	%rax, 16(%rsp)
	movq	$1, 24(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 48(%rsp)
	movq	%rax, 56(%rsp)
	movq	$8, 32(%rsp)
	movq	$0, 40(%rsp)
.Ltmp18:
	leaq	.L__unnamed_5(%rip), %rsi
	movq	_ZN4core9panicking9panic_fmt17hd1e5987b32e12cc9E@GOTPCREL(%rip), %rax
	leaq	16(%rsp), %rdi
	callq	*%rax
.Ltmp19:
	jmp	.LBB11_8
.LBB11_6:
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB11_7:
	.cfi_def_cfa_offset 80
.Ltmp20:
	movq	_ZN4core9panicking19panic_cannot_unwind17h4ffa2eadda6e2989E@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB11_8:
	ud2
.Lfunc_end11:
	.size	_ZN4core10intrinsics11write_bytes18precondition_check17h0e3014efe4c8c704E, .Lfunc_end11-_ZN4core10intrinsics11write_bytes18precondition_check17h0e3014efe4c8c704E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core10intrinsics11write_bytes18precondition_check17h0e3014efe4c8c704E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table11:
.Lexception1:
	.byte	255
	.byte	155
	.uleb128 .Lttbase1-.Lttbaseref1
.Lttbaseref1:
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Lfunc_begin1-.Lfunc_begin1
	.uleb128 .Ltmp18-.Lfunc_begin1
	.byte	0
	.byte	0
	.uleb128 .Ltmp18-.Lfunc_begin1
	.uleb128 .Ltmp19-.Ltmp18
	.uleb128 .Ltmp20-.Lfunc_begin1
	.byte	1
	.uleb128 .Ltmp19-.Lfunc_begin1
	.uleb128 .Lfunc_end11-.Ltmp19
	.byte	0
	.byte	0
.Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	.text._ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h46bc2dbafb3383c8E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h46bc2dbafb3383c8E,@function
_ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h46bc2dbafb3383c8E:
.Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception2
	subq	$152, %rsp
	.cfi_def_cfa_offset 160
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	%rdx, 24(%rsp)
	movq	%rcx, 32(%rsp)
	movq	%r8, 40(%rsp)
	cmpq	$0, %rdi
	jne	.LBB12_2
	jmp	.LBB12_3
.LBB12_2:
	movq	32(%rsp), %rcx
	movq	%rcx, %rax
	shrq	%rax
	movabsq	$6148914691236517205, %rdx
	andq	%rdx, %rax
	subq	%rax, %rcx
	movabsq	$3689348814741910323, %rdx
	movq	%rcx, %rax
	andq	%rdx, %rax
	shrq	$2, %rcx
	andq	%rdx, %rcx
	addq	%rcx, %rax
	movq	%rax, %rcx
	shrq	$4, %rcx
	addq	%rcx, %rax
	movabsq	$1085102592571150095, %rcx
	andq	%rcx, %rax
	movabsq	$72340172838076673, %rcx
	imulq	%rcx, %rax
	shrq	$56, %rax
	movl	%eax, 144(%rsp)
	cmpl	$1, 144(%rsp)
	je	.LBB12_4
	jmp	.LBB12_5
.LBB12_3:
	jmp	.LBB12_7
.LBB12_4:
	movq	8(%rsp), %rax
	movq	32(%rsp), %rcx
	subq	$1, %rcx
	andq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB12_6
	jmp	.LBB12_3
.LBB12_5:
	leaq	.L__unnamed_3(%rip), %rax
	movq	%rax, 48(%rsp)
	movq	$1, 56(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 80(%rsp)
	movq	%rax, 88(%rsp)
	movq	$8, 64(%rsp)
	movq	$0, 72(%rsp)
.Ltmp21:
	leaq	.L__unnamed_5(%rip), %rsi
	movq	_ZN4core9panicking9panic_fmt17hd1e5987b32e12cc9E@GOTPCREL(%rip), %rax
	leaq	48(%rsp), %rdi
	callq	*%rax
.Ltmp22:
	jmp	.LBB12_18
.LBB12_6:
	movq	16(%rsp), %rax
	cmpq	$0, %rax
	je	.LBB12_8
	jmp	.LBB12_9
.LBB12_7:
	leaq	.L__unnamed_6(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$166, %esi
	callq	*%rax
.LBB12_8:
	jmp	.LBB12_10
.LBB12_9:
	movq	32(%rsp), %rcx
	movq	%rcx, %rax
	shrq	%rax
	movabsq	$6148914691236517205, %rdx
	andq	%rdx, %rax
	subq	%rax, %rcx
	movabsq	$3689348814741910323, %rdx
	movq	%rcx, %rax
	andq	%rdx, %rax
	shrq	$2, %rcx
	andq	%rdx, %rcx
	addq	%rcx, %rax
	movq	%rax, %rcx
	shrq	$4, %rcx
	addq	%rcx, %rax
	movabsq	$1085102592571150095, %rcx
	andq	%rcx, %rax
	movabsq	$72340172838076673, %rcx
	imulq	%rcx, %rax
	shrq	$56, %rax
	movl	%eax, 148(%rsp)
	cmpl	$1, 148(%rsp)
	je	.LBB12_11
	jmp	.LBB12_12
.LBB12_10:
	jmp	.LBB12_7
.LBB12_11:
	movq	16(%rsp), %rax
	movq	32(%rsp), %rcx
	subq	$1, %rcx
	andq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB12_13
	jmp	.LBB12_10
.LBB12_12:
	leaq	.L__unnamed_3(%rip), %rax
	movq	%rax, 96(%rsp)
	movq	$1, 104(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 128(%rsp)
	movq	%rax, 136(%rsp)
	movq	$8, 112(%rsp)
	movq	$0, 120(%rsp)
.Ltmp23:
	leaq	.L__unnamed_5(%rip), %rsi
	movq	_ZN4core9panicking9panic_fmt17hd1e5987b32e12cc9E@GOTPCREL(%rip), %rax
	leaq	96(%rsp), %rdi
	callq	*%rax
.Ltmp24:
	jmp	.LBB12_18
.LBB12_13:
.Ltmp25:
	movq	40(%rsp), %rcx
	movq	24(%rsp), %rdx
	movq	16(%rsp), %rsi
	movq	8(%rsp), %rdi
	callq	_ZN4core9ub_checks17is_nonoverlapping7runtime17h69be12d802c2a34bE
.Ltmp26:
	movb	%al, 7(%rsp)
	jmp	.LBB12_15
.LBB12_14:
.Ltmp27:
	movq	_ZN4core9panicking19panic_cannot_unwind17h4ffa2eadda6e2989E@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB12_15:
	movb	7(%rsp), %al
	testb	$1, %al
	jne	.LBB12_17
	jmp	.LBB12_16
.LBB12_16:
	jmp	.LBB12_7
.LBB12_17:
	addq	$152, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB12_18:
	.cfi_def_cfa_offset 160
	ud2
.Lfunc_end12:
	.size	_ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h46bc2dbafb3383c8E, .Lfunc_end12-_ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h46bc2dbafb3383c8E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h46bc2dbafb3383c8E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table12:
.Lexception2:
	.byte	255
	.byte	155
	.uleb128 .Lttbase2-.Lttbaseref2
.Lttbaseref2:
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Ltmp21-.Lfunc_begin2
	.uleb128 .Ltmp22-.Ltmp21
	.uleb128 .Ltmp27-.Lfunc_begin2
	.byte	1
	.uleb128 .Ltmp22-.Lfunc_begin2
	.uleb128 .Ltmp23-.Ltmp22
	.byte	0
	.byte	0
	.uleb128 .Ltmp23-.Lfunc_begin2
	.uleb128 .Ltmp26-.Ltmp23
	.uleb128 .Ltmp27-.Lfunc_begin2
	.byte	1
	.uleb128 .Ltmp26-.Lfunc_begin2
	.uleb128 .Lfunc_end12-.Ltmp26
	.byte	0
	.byte	0
.Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	.text._ZN4core10intrinsics8unlikely17h8365633314ef2759E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core10intrinsics8unlikely17h8365633314ef2759E,@function
_ZN4core10intrinsics8unlikely17h8365633314ef2759E:
	.cfi_startproc
	movb	%dil, %al
	andb	$1, %al
	movzbl	%al, %eax
	retq
.Lfunc_end13:
	.size	_ZN4core10intrinsics8unlikely17h8365633314ef2759E, .Lfunc_end13-_ZN4core10intrinsics8unlikely17h8365633314ef2759E
	.cfi_endproc

	.section	".text._ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hf0a62e6cb88233a6E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hf0a62e6cb88233a6E,@function
_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hf0a62e6cb88233a6E:
	.cfi_startproc
	movq	(%rdi), %rcx
	movq	(%rsi), %rdx
	cmpq	%rdx, %rcx
	seta	%al
	andb	$1, %al
	cmpq	%rdx, %rcx
	setb	%cl
	andb	$1, %cl
	subb	%cl, %al
	retq
.Lfunc_end14:
	.size	_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hf0a62e6cb88233a6E, .Lfunc_end14-_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hf0a62e6cb88233a6E
	.cfi_endproc

	.section	".text._ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h96fbae46dc2208feE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h96fbae46dc2208feE,@function
_ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h96fbae46dc2208feE:
	.cfi_startproc
	movq	(%rdi), %rax
	cmpq	(%rsi), %rax
	sete	%al
	andb	$1, %al
	movzbl	%al, %eax
	retq
.Lfunc_end15:
	.size	_ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h96fbae46dc2208feE, .Lfunc_end15-_ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h96fbae46dc2208feE
	.cfi_endproc

	.section	.text._ZN4core3cmp6max_by17h9983db75b730376cE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3cmp6max_by17h9983db75b730376cE,@function
_ZN4core3cmp6max_by17h9983db75b730376cE:
.Lfunc_begin3:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception3
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movb	$1, 39(%rsp)
.Ltmp28:
	leaq	8(%rsp), %rdi
	leaq	16(%rsp), %rsi
	callq	_ZN4core3ops8function6FnOnce9call_once17hbc093a8e06adc12aE
.Ltmp29:
	movb	%al, 7(%rsp)
	jmp	.LBB16_3
.LBB16_1:
	jmp	.LBB16_9
.LBB16_2:
.Ltmp30:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 40(%rsp)
	movl	%eax, 48(%rsp)
	jmp	.LBB16_1
.LBB16_3:
	movb	7(%rsp), %al
	movb	%al, 38(%rsp)
	movb	38(%rsp), %al
	incb	%al
	subb	$2, %al
	jb	.LBB16_5
	jmp	.LBB16_12
.LBB16_12:
	jmp	.LBB16_6
	.cfi_def_cfa_offset 8
	ud2
.LBB16_5:
	.cfi_def_cfa_offset 64
	movq	16(%rsp), %rax
	movq	%rax, 24(%rsp)
	testb	$1, 39(%rsp)
	jne	.LBB16_8
	jmp	.LBB16_7
.LBB16_6:
	movb	$0, 39(%rsp)
	movq	8(%rsp), %rax
	movq	%rax, 24(%rsp)
.LBB16_7:
	movq	24(%rsp), %rax
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB16_8:
	.cfi_def_cfa_offset 64
	jmp	.LBB16_7
.LBB16_9:
	testb	$1, 39(%rsp)
	jne	.LBB16_11
.LBB16_10:
	movq	40(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB16_11:
	jmp	.LBB16_10
.Lfunc_end16:
	.size	_ZN4core3cmp6max_by17h9983db75b730376cE, .Lfunc_end16-_ZN4core3cmp6max_by17h9983db75b730376cE
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3cmp6max_by17h9983db75b730376cE,"a",@progbits
	.p2align	2, 0x0
GCC_except_table16:
.Lexception3:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end3-.Lcst_begin3
.Lcst_begin3:
	.uleb128 .Ltmp28-.Lfunc_begin3
	.uleb128 .Ltmp29-.Ltmp28
	.uleb128 .Ltmp30-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp29-.Lfunc_begin3
	.uleb128 .Lfunc_end16-.Ltmp29
	.byte	0
	.byte	0
.Lcst_end3:
	.p2align	2, 0x0

	.section	".text._ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h28b1788d2644cd66E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h28b1788d2644cd66E,@function
_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h28b1788d2644cd66E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, (%rsp)
	movq	%rsi, 8(%rsp)
	movl	52(%rsi), %eax
	andl	$16, %eax
	cmpl	$0, %eax
	jne	.LBB17_2
	movq	8(%rsp), %rax
	movl	52(%rax), %eax
	andl	$32, %eax
	cmpl	$0, %eax
	je	.LBB17_3
	jmp	.LBB17_4
.LBB17_2:
	movq	8(%rsp), %rsi
	movq	(%rsp), %rdi
	callq	*_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h5c009add778e56d2E@GOTPCREL(%rip)
	andb	$1, %al
	movb	%al, 23(%rsp)
	jmp	.LBB17_6
.LBB17_3:
	movq	8(%rsp), %rsi
	movq	(%rsp), %rdi
	callq	*_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hce8bd615af5e5030E@GOTPCREL(%rip)
	andb	$1, %al
	movb	%al, 23(%rsp)
	jmp	.LBB17_5
.LBB17_4:
	movq	8(%rsp), %rsi
	movq	(%rsp), %rdi
	callq	*_ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17h4ec775aff3eec867E@GOTPCREL(%rip)
	andb	$1, %al
	movb	%al, 23(%rsp)
.LBB17_5:
	jmp	.LBB17_6
.LBB17_6:
	movb	23(%rsp), %al
	andb	$1, %al
	movzbl	%al, %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end17:
	.size	_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h28b1788d2644cd66E, .Lfunc_end17-_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h28b1788d2644cd66E
	.cfi_endproc

	.section	.text._ZN4core3fmt9Arguments6new_v117heaacf47c44ec24ffE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3fmt9Arguments6new_v117heaacf47c44ec24ffE,@function
_ZN4core3fmt9Arguments6new_v117heaacf47c44ec24ffE:
	.cfi_startproc
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movq	%r8, 8(%rsp)
	movq	%rcx, 16(%rsp)
	movq	%rdx, 24(%rsp)
	movq	%rsi, 32(%rsp)
	movq	%rdi, 40(%rsp)
	movq	%rdi, 48(%rsp)
	cmpq	%r8, %rdx
	jb	.LBB18_2
	movq	24(%rsp), %rax
	movq	8(%rsp), %rcx
	addq	$1, %rcx
	cmpq	%rcx, %rax
	ja	.LBB18_4
	jmp	.LBB18_3
.LBB18_2:
	leaq	.L__unnamed_7(%rip), %rax
	movq	%rax, 56(%rsp)
	movq	$1, 64(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 88(%rsp)
	movq	%rax, 96(%rsp)
	movq	$8, 72(%rsp)
	movq	$0, 80(%rsp)
	leaq	.L__unnamed_8(%rip), %rsi
	movq	_ZN4core9panicking9panic_fmt17hd1e5987b32e12cc9E@GOTPCREL(%rip), %rax
	leaq	56(%rsp), %rdi
	callq	*%rax
.LBB18_3:
	movq	48(%rsp), %rax
	movq	40(%rsp), %rcx
	movq	8(%rsp), %rdx
	movq	16(%rsp), %rsi
	movq	24(%rsp), %rdi
	movq	32(%rsp), %r8
	movq	%r8, (%rcx)
	movq	%rdi, 8(%rcx)
	movq	.L__unnamed_4(%rip), %r8
	movq	.L__unnamed_4+8(%rip), %rdi
	movq	%r8, 32(%rcx)
	movq	%rdi, 40(%rcx)
	movq	%rsi, 16(%rcx)
	movq	%rdx, 24(%rcx)
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB18_4:
	.cfi_def_cfa_offset 112
	jmp	.LBB18_2
.Lfunc_end18:
	.size	_ZN4core3fmt9Arguments6new_v117heaacf47c44ec24ffE, .Lfunc_end18-_ZN4core3fmt9Arguments6new_v117heaacf47c44ec24ffE
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf6abef4c3f9244d6E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf6abef4c3f9244d6E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf6abef4c3f9244d6E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN4core3ops8function6FnOnce9call_once17h810440109cc76870E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end19:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf6abef4c3f9244d6E, .Lfunc_end19-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf6abef4c3f9244d6E
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17h810440109cc76870E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17h810440109cc76870E,@function
_ZN4core3ops8function6FnOnce9call_once17h810440109cc76870E:
.Lfunc_begin4:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception4
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
.Ltmp31:
	leaq	8(%rsp), %rdi
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E
.Ltmp32:
	movl	%eax, 4(%rsp)
	jmp	.LBB20_3
.LBB20_1:
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB20_2:
.Ltmp33:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 24(%rsp)
	movl	%eax, 32(%rsp)
	jmp	.LBB20_1
.LBB20_3:
	movl	4(%rsp), %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end20:
	.size	_ZN4core3ops8function6FnOnce9call_once17h810440109cc76870E, .Lfunc_end20-_ZN4core3ops8function6FnOnce9call_once17h810440109cc76870E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3ops8function6FnOnce9call_once17h810440109cc76870E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table20:
.Lexception4:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end4-.Lcst_begin4
.Lcst_begin4:
	.uleb128 .Ltmp31-.Lfunc_begin4
	.uleb128 .Ltmp32-.Ltmp31
	.uleb128 .Ltmp33-.Lfunc_begin4
	.byte	0
	.uleb128 .Ltmp32-.Lfunc_begin4
	.uleb128 .Lfunc_end20-.Ltmp32
	.byte	0
	.byte	0
.Lcst_end4:
	.p2align	2, 0x0

	.section	.text._ZN4core3ops8function6FnOnce9call_once17hbc093a8e06adc12aE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17hbc093a8e06adc12aE,@function
_ZN4core3ops8function6FnOnce9call_once17hbc093a8e06adc12aE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	callq	_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hf0a62e6cb88233a6E
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end21:
	.size	_ZN4core3ops8function6FnOnce9call_once17hbc093a8e06adc12aE, .Lfunc_end21-_ZN4core3ops8function6FnOnce9call_once17hbc093a8e06adc12aE
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17hc9488438d58459e6E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17hc9488438d58459e6E,@function
_ZN4core3ops8function6FnOnce9call_once17hc9488438d58459e6E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end22:
	.size	_ZN4core3ops8function6FnOnce9call_once17hc9488438d58459e6E, .Lfunc_end22-_ZN4core3ops8function6FnOnce9call_once17hc9488438d58459e6E
	.cfi_endproc

	.section	.text._ZN4core3ptr13read_volatile18precondition_check17h7476b258d0cecb32E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr13read_volatile18precondition_check17h7476b258d0cecb32E,@function
_ZN4core3ptr13read_volatile18precondition_check17h7476b258d0cecb32E:
.Lfunc_begin5:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception5
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rdi, (%rsp)
	movq	%rsi, 8(%rsp)
	cmpq	$0, %rdi
	jne	.LBB23_2
	jmp	.LBB23_3
.LBB23_2:
	movq	8(%rsp), %rcx
	movq	%rcx, %rax
	shrq	%rax
	movabsq	$6148914691236517205, %rdx
	andq	%rdx, %rax
	subq	%rax, %rcx
	movabsq	$3689348814741910323, %rdx
	movq	%rcx, %rax
	andq	%rdx, %rax
	shrq	$2, %rcx
	andq	%rdx, %rcx
	addq	%rcx, %rax
	movq	%rax, %rcx
	shrq	$4, %rcx
	addq	%rcx, %rax
	movabsq	$1085102592571150095, %rcx
	andq	%rcx, %rax
	movabsq	$72340172838076673, %rcx
	imulq	%rcx, %rax
	shrq	$56, %rax
	movl	%eax, 68(%rsp)
	cmpl	$1, 68(%rsp)
	je	.LBB23_4
	jmp	.LBB23_5
.LBB23_3:
	leaq	.L__unnamed_9(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$110, %esi
	callq	*%rax
.LBB23_4:
	movq	(%rsp), %rax
	movq	8(%rsp), %rcx
	subq	$1, %rcx
	andq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB23_6
	jmp	.LBB23_3
.LBB23_5:
	leaq	.L__unnamed_3(%rip), %rax
	movq	%rax, 16(%rsp)
	movq	$1, 24(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 48(%rsp)
	movq	%rax, 56(%rsp)
	movq	$8, 32(%rsp)
	movq	$0, 40(%rsp)
.Ltmp34:
	leaq	.L__unnamed_5(%rip), %rsi
	movq	_ZN4core9panicking9panic_fmt17hd1e5987b32e12cc9E@GOTPCREL(%rip), %rax
	leaq	16(%rsp), %rdi
	callq	*%rax
.Ltmp35:
	jmp	.LBB23_8
.LBB23_6:
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB23_7:
	.cfi_def_cfa_offset 80
.Ltmp36:
	movq	_ZN4core9panicking19panic_cannot_unwind17h4ffa2eadda6e2989E@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB23_8:
	ud2
.Lfunc_end23:
	.size	_ZN4core3ptr13read_volatile18precondition_check17h7476b258d0cecb32E, .Lfunc_end23-_ZN4core3ptr13read_volatile18precondition_check17h7476b258d0cecb32E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3ptr13read_volatile18precondition_check17h7476b258d0cecb32E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table23:
.Lexception5:
	.byte	255
	.byte	155
	.uleb128 .Lttbase3-.Lttbaseref3
.Lttbaseref3:
	.byte	1
	.uleb128 .Lcst_end5-.Lcst_begin5
.Lcst_begin5:
	.uleb128 .Lfunc_begin5-.Lfunc_begin5
	.uleb128 .Ltmp34-.Lfunc_begin5
	.byte	0
	.byte	0
	.uleb128 .Ltmp34-.Lfunc_begin5
	.uleb128 .Ltmp35-.Ltmp34
	.uleb128 .Ltmp36-.Lfunc_begin5
	.byte	1
	.uleb128 .Ltmp35-.Lfunc_begin5
	.uleb128 .Lfunc_end23-.Ltmp35
	.byte	0
	.byte	0
.Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	".text._ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hf20ba47d8a25a9f3E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hf20ba47d8a25a9f3E,@function
_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hf20ba47d8a25a9f3E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN157_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0fe6586ba72b043bE
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end24:
	.size	_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hf20ba47d8a25a9f3E, .Lfunc_end24-_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hf20ba47d8a25a9f3E
	.cfi_endproc

	.section	".text._ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17h2b4d751a3ac2caa0E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17h2b4d751a3ac2caa0E,@function
_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17h2b4d751a3ac2caa0E:
	.cfi_startproc
	retq
.Lfunc_end25:
	.size	_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17h2b4d751a3ac2caa0E, .Lfunc_end25-_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17h2b4d751a3ac2caa0E
	.cfi_endproc

	.section	".text._ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7e77b91b8b0668beE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7e77b91b8b0668beE,@function
_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7e77b91b8b0668beE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h5b4791c21dcf190bE
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end26:
	.size	_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7e77b91b8b0668beE, .Lfunc_end26-_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7e77b91b8b0668beE
	.cfi_endproc

	.section	".text._ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h5b4791c21dcf190bE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h5b4791c21dcf190bE,@function
_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h5b4791c21dcf190bE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ptr52drop_in_place$LT$std..sys..pal..unix..args..Args$GT$17haf3f92a511d20e06E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end27:
	.size	_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h5b4791c21dcf190bE, .Lfunc_end27-_ZN4core3ptr37drop_in_place$LT$std..env..ArgsOs$GT$17h5b4791c21dcf190bE
	.cfi_endproc

	.section	".text._ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE,@function
_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end28:
	.size	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE, .Lfunc_end28-_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE
	.cfi_endproc

	.section	".text._ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E,@function
_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E:
.Lfunc_begin6:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception6
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, (%rsp)
.Ltmp37:
	callq	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d9ede7d8b542cd0E
.Ltmp38:
	jmp	.LBB29_3
.LBB29_1:
.Ltmp40:
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc1f1fcdbc089722dE
.Ltmp41:
	jmp	.LBB29_5
.LBB29_2:
.Ltmp39:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 8(%rsp)
	movl	%eax, 16(%rsp)
	jmp	.LBB29_1
.LBB29_3:
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc1f1fcdbc089722dE
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB29_4:
	.cfi_def_cfa_offset 32
.Ltmp42:
	movq	_ZN4core9panicking16panic_in_cleanup17he962919aa9bfab8cE@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB29_5:
	movq	8(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end29:
	.size	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E, .Lfunc_end29-_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E","a",@progbits
	.p2align	2, 0x0
GCC_except_table29:
.Lexception6:
	.byte	255
	.byte	155
	.uleb128 .Lttbase4-.Lttbaseref4
.Lttbaseref4:
	.byte	1
	.uleb128 .Lcst_end6-.Lcst_begin6
.Lcst_begin6:
	.uleb128 .Ltmp37-.Lfunc_begin6
	.uleb128 .Ltmp38-.Ltmp37
	.uleb128 .Ltmp39-.Lfunc_begin6
	.byte	0
	.uleb128 .Ltmp40-.Lfunc_begin6
	.uleb128 .Ltmp41-.Ltmp40
	.uleb128 .Ltmp42-.Lfunc_begin6
	.byte	1
	.uleb128 .Ltmp41-.Lfunc_begin6
	.uleb128 .Lfunc_end29-.Ltmp41
	.byte	0
	.byte	0
.Lcst_end6:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.section	".text._ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17heb97595a1e98dbc6E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17heb97595a1e98dbc6E,@function
_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17heb97595a1e98dbc6E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h1236b8083be10637E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end30:
	.size	_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17heb97595a1e98dbc6E, .Lfunc_end30-_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17heb97595a1e98dbc6E
	.cfi_endproc

	.section	".text._ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE,@function
_ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE:
.Lfunc_begin7:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception7
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, (%rsp)
.Ltmp43:
	callq	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4019c5a79aa8b0f1E
.Ltmp44:
	jmp	.LBB31_3
.LBB31_1:
.Ltmp46:
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr55drop_in_place$LT$alloc..raw_vec..RawVec$LT$char$GT$$GT$17h2a7887598f58ad60E
.Ltmp47:
	jmp	.LBB31_5
.LBB31_2:
.Ltmp45:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 8(%rsp)
	movl	%eax, 16(%rsp)
	jmp	.LBB31_1
.LBB31_3:
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr55drop_in_place$LT$alloc..raw_vec..RawVec$LT$char$GT$$GT$17h2a7887598f58ad60E
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB31_4:
	.cfi_def_cfa_offset 32
.Ltmp48:
	movq	_ZN4core9panicking16panic_in_cleanup17he962919aa9bfab8cE@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB31_5:
	movq	8(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end31:
	.size	_ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE, .Lfunc_end31-_ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE","a",@progbits
	.p2align	2, 0x0
GCC_except_table31:
.Lexception7:
	.byte	255
	.byte	155
	.uleb128 .Lttbase5-.Lttbaseref5
.Lttbaseref5:
	.byte	1
	.uleb128 .Lcst_end7-.Lcst_begin7
.Lcst_begin7:
	.uleb128 .Ltmp43-.Lfunc_begin7
	.uleb128 .Ltmp44-.Ltmp43
	.uleb128 .Ltmp45-.Lfunc_begin7
	.byte	0
	.uleb128 .Ltmp46-.Lfunc_begin7
	.uleb128 .Ltmp47-.Ltmp46
	.uleb128 .Ltmp48-.Lfunc_begin7
	.byte	1
	.uleb128 .Ltmp47-.Lfunc_begin7
	.uleb128 .Lfunc_end31-.Ltmp47
	.byte	0
	.byte	0
.Lcst_end7:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase5:
	.byte	0
	.p2align	2, 0x0

	.section	".text._ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h1236b8083be10637E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h1236b8083be10637E,@function
_ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h1236b8083be10637E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h2b9b46263571fc42E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end32:
	.size	_ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h1236b8083be10637E, .Lfunc_end32-_ZN4core3ptr49drop_in_place$LT$std..sys..os_str..bytes..Buf$GT$17h1236b8083be10637E
	.cfi_endproc

	.section	".text._ZN4core3ptr52drop_in_place$LT$std..sys..pal..unix..args..Args$GT$17haf3f92a511d20e06E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr52drop_in_place$LT$std..sys..pal..unix..args..Args$GT$17haf3f92a511d20e06E,@function
_ZN4core3ptr52drop_in_place$LT$std..sys..pal..unix..args..Args$GT$17haf3f92a511d20e06E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17hf6bd7982db6a1ab5E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end33:
	.size	_ZN4core3ptr52drop_in_place$LT$std..sys..pal..unix..args..Args$GT$17haf3f92a511d20e06E, .Lfunc_end33-_ZN4core3ptr52drop_in_place$LT$std..sys..pal..unix..args..Args$GT$17haf3f92a511d20e06E
	.cfi_endproc

	.section	".text._ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc1f1fcdbc089722dE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc1f1fcdbc089722dE,@function
_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc1f1fcdbc089722dE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1fe377d573639452E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end34:
	.size	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc1f1fcdbc089722dE, .Lfunc_end34-_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc1f1fcdbc089722dE
	.cfi_endproc

	.section	".text._ZN4core3ptr55drop_in_place$LT$alloc..raw_vec..RawVec$LT$char$GT$$GT$17h2a7887598f58ad60E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr55drop_in_place$LT$alloc..raw_vec..RawVec$LT$char$GT$$GT$17h2a7887598f58ad60E,@function
_ZN4core3ptr55drop_in_place$LT$alloc..raw_vec..RawVec$LT$char$GT$$GT$17h2a7887598f58ad60E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbe00bea1188b9e66E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end35:
	.size	_ZN4core3ptr55drop_in_place$LT$alloc..raw_vec..RawVec$LT$char$GT$$GT$17h2a7887598f58ad60E, .Lfunc_end35-_ZN4core3ptr55drop_in_place$LT$alloc..raw_vec..RawVec$LT$char$GT$$GT$17h2a7887598f58ad60E
	.cfi_endproc

	.section	".text._ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h162bc891fdd82777E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h162bc891fdd82777E,@function
_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h162bc891fdd82777E:
.Lfunc_begin8:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception8
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, (%rsp)
	movq	%rsi, 8(%rsp)
	movq	$0, 16(%rsp)
.LBB36_1:
	movq	8(%rsp), %rax
	cmpq	%rax, 16(%rsp)
	je	.LBB36_3
	movq	(%rsp), %rcx
	movq	16(%rsp), %rax
	leaq	(%rax,%rax,2), %rdx
	leaq	(%rcx,%rdx,8), %rdi
	incq	%rax
	movq	%rax, 16(%rsp)
.Ltmp49:
	callq	_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17heb97595a1e98dbc6E
.Ltmp50:
	jmp	.LBB36_1
.LBB36_3:
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB36_4:
	.cfi_def_cfa_offset 48
	movq	8(%rsp), %rax
	cmpq	%rax, 16(%rsp)
	je	.LBB36_7
	jmp	.LBB36_6
.LBB36_5:
.Ltmp51:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 24(%rsp)
	movl	%eax, 32(%rsp)
	jmp	.LBB36_4
.LBB36_6:
	movq	(%rsp), %rcx
	movq	16(%rsp), %rax
	leaq	(%rax,%rax,2), %rdx
	leaq	(%rcx,%rdx,8), %rdi
	incq	%rax
	movq	%rax, 16(%rsp)
.Ltmp52:
	callq	_ZN4core3ptr47drop_in_place$LT$std..ffi..os_str..OsString$GT$17heb97595a1e98dbc6E
.Ltmp53:
	jmp	.LBB36_4
.LBB36_7:
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB36_8:
.Ltmp54:
	movq	_ZN4core9panicking16panic_in_cleanup17he962919aa9bfab8cE@GOTPCREL(%rip), %rax
	callq	*%rax
.Lfunc_end36:
	.size	_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h162bc891fdd82777E, .Lfunc_end36-_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h162bc891fdd82777E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h162bc891fdd82777E","a",@progbits
	.p2align	2, 0x0
GCC_except_table36:
.Lexception8:
	.byte	255
	.byte	155
	.uleb128 .Lttbase6-.Lttbaseref6
.Lttbaseref6:
	.byte	1
	.uleb128 .Lcst_end8-.Lcst_begin8
.Lcst_begin8:
	.uleb128 .Ltmp49-.Lfunc_begin8
	.uleb128 .Ltmp50-.Ltmp49
	.uleb128 .Ltmp51-.Lfunc_begin8
	.byte	0
	.uleb128 .Ltmp52-.Lfunc_begin8
	.uleb128 .Ltmp53-.Ltmp52
	.uleb128 .Ltmp54-.Lfunc_begin8
	.byte	1
	.uleb128 .Ltmp53-.Lfunc_begin8
	.uleb128 .Lfunc_end36-.Ltmp53
	.byte	0
	.byte	0
.Lcst_end8:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase6:
	.byte	0
	.p2align	2, 0x0

	.section	".text._ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h596649ad4273d3e1E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h596649ad4273d3e1E,@function
_ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h596649ad4273d3e1E:
	.cfi_startproc
	retq
.Lfunc_end37:
	.size	_ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h596649ad4273d3e1E, .Lfunc_end37-_ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h596649ad4273d3e1E
	.cfi_endproc

	.section	".text._ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17hc4e56f9fd9e5dbc2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17hc4e56f9fd9e5dbc2E,@function
_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17hc4e56f9fd9e5dbc2E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, (%rsp)
	movabsq	$-9223372036854775808, %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	%rdx, (%rdi)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB38_2
.LBB38_1:
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.LBB38_2:
	.cfi_def_cfa_offset 16
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE
	jmp	.LBB38_1
.Lfunc_end38:
	.size	_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17hc4e56f9fd9e5dbc2E, .Lfunc_end38-_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17hc4e56f9fd9e5dbc2E
	.cfi_endproc

	.section	".text._ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17hf0d179b9f1cf3ebaE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17hf0d179b9f1cf3ebaE,@function
_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17hf0d179b9f1cf3ebaE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hac1707515fd3d6beE
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end39:
	.size	_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17hf0d179b9f1cf3ebaE, .Lfunc_end39-_ZN4core3ptr77drop_in_place$LT$alloc..raw_vec..RawVec$LT$std..ffi..os_str..OsString$GT$$GT$17hf0d179b9f1cf3ebaE
	.cfi_endproc

	.section	".text._ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7a4c5a107acd5ab2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7a4c5a107acd5ab2E,@function
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7a4c5a107acd5ab2E:
	.cfi_startproc
	retq
.Lfunc_end40:
	.size	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7a4c5a107acd5ab2E, .Lfunc_end40-_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7a4c5a107acd5ab2E
	.cfi_endproc

	.section	".text._ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17hf6bd7982db6a1ab5E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17hf6bd7982db6a1ab5E,@function
_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17hf6bd7982db6a1ab5E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17haf3e26ebdf106e32E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end41:
	.size	_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17hf6bd7982db6a1ab5E, .Lfunc_end41-_ZN4core3ptr86drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$std..ffi..os_str..OsString$GT$$GT$17hf6bd7982db6a1ab5E
	.cfi_endproc

	.section	".text._ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E,@function
_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpq	$0, %rdi
	jne	.LBB42_2
	leaq	.L__unnamed_10(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$93, %esi
	callq	*%rax
.LBB42_2:
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end42:
	.size	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E, .Lfunc_end42-_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	.cfi_endproc

	.section	".text._ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h4f8c88dcc015f813E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h4f8c88dcc015f813E,@function
_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h4f8c88dcc015f813E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, (%rsp)
	movq	%rsi, 8(%rsp)
	movq	8(%rsp), %rsi
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr18precondition_check17h004ce445661bfd41E
	jmp	.LBB43_3
.LBB43_3:
	jmp	.LBB43_4
.LBB43_4:
	movq	8(%rsp), %rcx
	movq	(%rsp), %rax
	subq	%rcx, %rax
	movl	$24, %ecx
	xorl	%edx, %edx
	divq	%rcx
	movq	%rax, 16(%rsp)
	movq	16(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end43:
	.size	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h4f8c88dcc015f813E, .Lfunc_end43-_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h4f8c88dcc015f813E
	.cfi_endproc

	.section	".text._ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17hfb875bf6c2b65649E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17hfb875bf6c2b65649E,@function
_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17hfb875bf6c2b65649E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, (%rsp)
	movq	%rsi, 8(%rsp)
	movq	8(%rsp), %rsi
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr18precondition_check17h004ce445661bfd41E
	jmp	.LBB44_3
.LBB44_3:
	jmp	.LBB44_4
.LBB44_4:
	movq	8(%rsp), %rcx
	movq	(%rsp), %rax
	subq	%rcx, %rax
	shrq	$0, %rax
	movq	%rax, 16(%rsp)
	movq	16(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end44:
	.size	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17hfb875bf6c2b65649E, .Lfunc_end44-_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17hfb875bf6c2b65649E
	.cfi_endproc

	.section	".text._ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr18precondition_check17h004ce445661bfd41E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr18precondition_check17h004ce445661bfd41E,@function
_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr18precondition_check17h004ce445661bfd41E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpq	%rsi, %rdi
	jae	.LBB45_2
	leaq	.L__unnamed_11(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$71, %esi
	callq	*%rax
.LBB45_2:
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end45:
	.size	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr18precondition_check17h004ce445661bfd41E, .Lfunc_end45-_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr18precondition_check17h004ce445661bfd41E
	.cfi_endproc

	.section	.text._ZN4core3str11validations15next_code_point17h3c18ed530c8364b9E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3str11validations15next_code_point17h3c18ed530c8364b9E,@function
_ZN4core3str11validations15next_code_point17h3c18ed530c8364b9E:
	.cfi_startproc
	subq	$88, %rsp
	.cfi_def_cfa_offset 96
	movq	%rdi, 24(%rsp)
	callq	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E
	movq	%rax, 48(%rsp)
	movq	48(%rsp), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB46_2
	movl	.L__unnamed_12(%rip), %ecx
	movl	.L__unnamed_12+4(%rip), %eax
	movl	%ecx, 32(%rsp)
	movl	%eax, 36(%rsp)
	jmp	.LBB46_3
.LBB46_2:
	movq	48(%rsp), %rax
	movq	%rax, 40(%rsp)
	movq	40(%rsp), %rax
	movb	(%rax), %al
	movb	%al, 23(%rsp)
	cmpb	$-128, %al
	jb	.LBB46_5
	jmp	.LBB46_4
.LBB46_3:
	movl	32(%rsp), %eax
	movl	36(%rsp), %edx
	addq	$88, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB46_4:
	.cfi_def_cfa_offset 96
	movq	24(%rsp), %rdi
	movb	23(%rsp), %al
	andb	$31, %al
	movzbl	%al, %eax
	movl	%eax, 16(%rsp)
	callq	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E
	movq	%rax, 56(%rsp)
	movq	56(%rsp), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB46_6
	jmp	.LBB46_7
.LBB46_5:
	movb	23(%rsp), %al
	movzbl	%al, %eax
	movl	%eax, 36(%rsp)
	movl	$1, 32(%rsp)
	jmp	.LBB46_3
.LBB46_6:
	callq	_ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E
	jmp	.LBB46_8
.LBB46_7:
	movb	23(%rsp), %al
	movl	16(%rsp), %ecx
	movq	56(%rsp), %rdx
	movb	(%rdx), %dl
	shll	$6, %ecx
	andb	$63, %dl
	movzbl	%dl, %edx
	movl	%edx, 12(%rsp)
	orl	%edx, %ecx
	movl	%ecx, 68(%rsp)
	cmpb	$-32, %al
	jae	.LBB46_10
	jmp	.LBB46_9
.LBB46_8:
	ud2
.LBB46_9:
	movl	68(%rsp), %eax
	movl	%eax, 36(%rsp)
	movl	$1, 32(%rsp)
	jmp	.LBB46_3
.LBB46_10:
	movq	24(%rsp), %rdi
	callq	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E
	movq	%rax, 72(%rsp)
	movq	72(%rsp), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB46_12
	callq	_ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E
	jmp	.LBB46_8
.LBB46_12:
	movb	23(%rsp), %al
	movl	16(%rsp), %ecx
	movl	12(%rsp), %edx
	movq	72(%rsp), %rsi
	movb	(%rsi), %sil
	shll	$6, %edx
	andb	$63, %sil
	movzbl	%sil, %esi
	orl	%esi, %edx
	movl	%edx, 8(%rsp)
	shll	$12, %ecx
	orl	%edx, %ecx
	movl	%ecx, 68(%rsp)
	cmpb	$-16, %al
	jae	.LBB46_14
.LBB46_13:
	jmp	.LBB46_9
.LBB46_14:
	movq	24(%rsp), %rdi
	callq	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E
	movq	%rax, 80(%rsp)
	movq	80(%rsp), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB46_16
	callq	_ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E
	jmp	.LBB46_8
.LBB46_16:
	movl	8(%rsp), %ecx
	movl	16(%rsp), %eax
	movq	80(%rsp), %rdx
	movb	(%rdx), %dl
	andl	$7, %eax
	shll	$18, %eax
	shll	$6, %ecx
	andb	$63, %dl
	movzbl	%dl, %edx
	orl	%edx, %ecx
	orl	%ecx, %eax
	movl	%eax, 68(%rsp)
	jmp	.LBB46_13
.Lfunc_end46:
	.size	_ZN4core3str11validations15next_code_point17h3c18ed530c8364b9E, .Lfunc_end46-_ZN4core3str11validations15next_code_point17h3c18ed530c8364b9E
	.cfi_endproc

	.section	".text._ZN4core3str21_$LT$impl$u20$str$GT$5chars17hc60612dfa7c10438E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3str21_$LT$impl$u20$str$GT$5chars17hc60612dfa7c10438E,@function
_ZN4core3str21_$LT$impl$u20$str$GT$5chars17hc60612dfa7c10438E:
	.cfi_startproc
	movq	%rdi, %rax
	movq	%rax, %rdx
	addq	%rsi, %rdx
	retq
.Lfunc_end47:
	.size	_ZN4core3str21_$LT$impl$u20$str$GT$5chars17hc60612dfa7c10438E, .Lfunc_end47-_ZN4core3str21_$LT$impl$u20$str$GT$5chars17hc60612dfa7c10438E
	.cfi_endproc

	.section	.text._ZN4core4char7convert18from_u32_unchecked18precondition_check17hd735cb21d94032c4E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core4char7convert18from_u32_unchecked18precondition_check17hd735cb21d94032c4E,@function
_ZN4core4char7convert18from_u32_unchecked18precondition_check17hd735cb21d94032c4E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	%edi, %eax
	movl	%eax, (%rsp)
	xorl	$55296, %eax
	subl	$2048, %eax
	cmpl	$1112064, %eax
	jae	.LBB48_2
	movl	(%rsp), %eax
	movl	%eax, 4(%rsp)
	jmp	.LBB48_3
.LBB48_2:
	movl	$1114112, 4(%rsp)
.LBB48_3:
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpl	$1114112, 4(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB48_5
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.LBB48_5:
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_13(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$57, %esi
	callq	*%rax
.Lfunc_end48:
	.size	_ZN4core4char7convert18from_u32_unchecked18precondition_check17hd735cb21d94032c4E, .Lfunc_end48-_ZN4core4char7convert18from_u32_unchecked18precondition_check17hd735cb21d94032c4E
	.cfi_endproc

	.section	.text._ZN4core4hint16assert_unchecked18precondition_check17h37afe919f80c4ea4E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core4hint16assert_unchecked18precondition_check17h37afe919f80c4ea4E,@function
_ZN4core4hint16assert_unchecked18precondition_check17h37afe919f80c4ea4E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movb	%dil, %al
	testb	$1, %al
	jne	.LBB49_2
	jmp	.LBB49_1
.LBB49_1:
	leaq	.L__unnamed_14(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$104, %esi
	callq	*%rax
.LBB49_2:
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end49:
	.size	_ZN4core4hint16assert_unchecked18precondition_check17h37afe919f80c4ea4E, .Lfunc_end49-_ZN4core4hint16assert_unchecked18precondition_check17h37afe919f80c4ea4E
	.cfi_endproc

	.section	.text._ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E,@function
_ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	leaq	.L__unnamed_15(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$82, %esi
	callq	*%rax
.Lfunc_end50:
	.size	_ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E, .Lfunc_end50-_ZN4core4hint21unreachable_unchecked18precondition_check17h91aacf12e4279540E
	.cfi_endproc

	.section	.text._ZN4core4iter6traits10exact_size17ExactSizeIterator3len17hb2f10e08cbf9afa9E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17hb2f10e08cbf9afa9E,@function
_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17hb2f10e08cbf9afa9E:
	.cfi_startproc
	subq	$120, %rsp
	.cfi_def_cfa_offset 128
	movq	%rdi, %rsi
	leaq	32(%rsp), %rdi
	callq	_ZN103_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17hb6bb605b6728e365E
	movq	32(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	40(%rsp), %rdx
	movq	48(%rsp), %rcx
	movq	%rdx, 16(%rsp)
	movq	%rcx, 24(%rsp)
	movq	%rax, 64(%rsp)
	movq	$1, 56(%rsp)
	leaq	16(%rsp), %rdi
	leaq	56(%rsp), %rsi
	callq	_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17he00a9f76f4e9dd8aE
	testb	$1, %al
	jne	.LBB51_2
	movq	$0, 72(%rsp)
	leaq	.L__unnamed_16(%rip), %r8
	xorl	%edi, %edi
	leaq	16(%rsp), %rsi
	leaq	56(%rsp), %rdx
	leaq	72(%rsp), %rcx
	callq	_ZN4core9panicking13assert_failed17h3576a882b3343974E
.LBB51_2:
	movq	8(%rsp), %rax
	addq	$120, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end51:
	.size	_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17hb2f10e08cbf9afa9E, .Lfunc_end51-_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17hb2f10e08cbf9afa9E
	.cfi_endproc

	.section	.text._ZN4core4iter6traits8iterator8Iterator7collect17h261c26de2281c052E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core4iter6traits8iterator8Iterator7collect17h261c26de2281c052E,@function
_ZN4core4iter6traits8iterator8Iterator7collect17h261c26de2281c052E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, %rax
	movq	%rax, (%rsp)
	callq	_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h23d53d60e1f7f8c3E
	movq	(%rsp), %rax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end52:
	.size	_ZN4core4iter6traits8iterator8Iterator7collect17h261c26de2281c052E, .Lfunc_end52-_ZN4core4iter6traits8iterator8Iterator7collect17h261c26de2281c052E
	.cfi_endproc

	.section	.text._ZN4core5alloc6layout6Layout5array5inner17he6fccdbabe27881dE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core5alloc6layout6Layout5array5inner17he6fccdbabe27881dE,@function
_ZN4core5alloc6layout6Layout5array5inner17he6fccdbabe27881dE:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rdi, 16(%rsp)
	movq	%rsi, 24(%rsp)
	movq	%rdx, 32(%rsp)
	cmpq	$0, %rdi
	jne	.LBB53_2
.LBB53_1:
	movq	24(%rsp), %rcx
	movq	32(%rsp), %rdx
	movq	16(%rsp), %rax
	imulq	%rdx, %rax
	movq	%rcx, 64(%rsp)
	movq	64(%rsp), %rcx
	movq	%rcx, 40(%rsp)
	movq	%rax, 48(%rsp)
	jmp	.LBB53_7
.LBB53_2:
	movq	16(%rsp), %rax
	movq	24(%rsp), %rcx
	movq	%rcx, 56(%rsp)
	movq	56(%rsp), %rdx
	subq	$1, %rdx
	movabsq	$9223372036854775807, %rcx
	subq	%rdx, %rcx
	movq	%rcx, 8(%rsp)
	cmpq	$0, %rax
	je	.LBB53_4
	movq	16(%rsp), %rcx
	movq	8(%rsp), %rax
	xorl	%edx, %edx
	divq	%rcx
	movq	%rax, %rcx
	movq	32(%rsp), %rax
	cmpq	%rcx, %rax
	ja	.LBB53_6
	jmp	.LBB53_5
.LBB53_4:
	leaq	.L__unnamed_17(%rip), %rdi
	movq	_ZN4core9panicking11panic_const23panic_const_div_by_zero17h8f429c9723ae7081E@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB53_5:
	jmp	.LBB53_1
.LBB53_6:
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 40(%rsp)
	movq	%rax, 48(%rsp)
.LBB53_7:
	movq	40(%rsp), %rax
	movq	48(%rsp), %rdx
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end53:
	.size	_ZN4core5alloc6layout6Layout5array5inner17he6fccdbabe27881dE, .Lfunc_end53-_ZN4core5alloc6layout6Layout5array5inner17he6fccdbabe27881dE
	.cfi_endproc

	.section	.text._ZN4core5alloc6layout6Layout8dangling17h4092b41d461313f7E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core5alloc6layout6Layout8dangling17h4092b41d461313f7E,@function
_ZN4core5alloc6layout6Layout8dangling17h4092b41d461313f7E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	(%rdi), %rax
	movq	%rax, 16(%rsp)
	movq	16(%rsp), %rcx
	xorl	%eax, %eax
	addq	%rcx, %rax
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rdi
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end54:
	.size	_ZN4core5alloc6layout6Layout8dangling17h4092b41d461313f7E, .Lfunc_end54-_ZN4core5alloc6layout6Layout8dangling17h4092b41d461313f7E
	.cfi_endproc

	.section	".text._ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h122b5a9490b52eb2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h122b5a9490b52eb2E,@function
_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h122b5a9490b52eb2E:
	.cfi_startproc
	movq	%rdi, -24(%rsp)
	movq	%rsi, -16(%rsp)
	movq	-24(%rsp), %rax
	movq	-16(%rsp), %rcx
	shlq	$2, %rcx
	addq	%rcx, %rax
	movq	%rax, -8(%rsp)
	movq	-24(%rsp), %rax
	movq	-8(%rsp), %rdx
	retq
.Lfunc_end55:
	.size	_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h122b5a9490b52eb2E, .Lfunc_end55-_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h122b5a9490b52eb2E
	.cfi_endproc

	.section	.text._ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E,@function
_ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E:
.Lfunc_begin9:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception9
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	%rdx, 24(%rsp)
	movq	%rcx, 32(%rsp)
	cmpq	$0, %rdi
	jne	.LBB56_2
	jmp	.LBB56_3
.LBB56_2:
	movq	24(%rsp), %rcx
	movq	%rcx, %rax
	shrq	%rax
	movabsq	$6148914691236517205, %rdx
	andq	%rdx, %rax
	subq	%rax, %rcx
	movabsq	$3689348814741910323, %rdx
	movq	%rcx, %rax
	andq	%rdx, %rax
	shrq	$2, %rcx
	andq	%rdx, %rcx
	addq	%rcx, %rax
	movq	%rax, %rcx
	shrq	$4, %rcx
	addq	%rcx, %rax
	movabsq	$1085102592571150095, %rcx
	andq	%rcx, %rax
	movabsq	$72340172838076673, %rcx
	imulq	%rcx, %rax
	shrq	$56, %rax
	movl	%eax, 100(%rsp)
	cmpl	$1, 100(%rsp)
	je	.LBB56_4
	jmp	.LBB56_5
.LBB56_3:
	jmp	.LBB56_7
.LBB56_4:
	movq	8(%rsp), %rax
	movq	24(%rsp), %rcx
	subq	$1, %rcx
	andq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB56_6
	jmp	.LBB56_3
.LBB56_5:
	leaq	.L__unnamed_3(%rip), %rax
	movq	%rax, 40(%rsp)
	movq	$1, 48(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 72(%rsp)
	movq	%rax, 80(%rsp)
	movq	$8, 56(%rsp)
	movq	$0, 64(%rsp)
.Ltmp55:
	leaq	.L__unnamed_5(%rip), %rsi
	movq	_ZN4core9panicking9panic_fmt17hd1e5987b32e12cc9E@GOTPCREL(%rip), %rax
	leaq	40(%rsp), %rdi
	callq	*%rax
.Ltmp56:
	jmp	.LBB56_14
.LBB56_6:
	movq	16(%rsp), %rax
	cmpq	$0, %rax
	sete	%cl
	movb	%cl, 7(%rsp)
	cmpq	$0, %rax
	je	.LBB56_8
	jmp	.LBB56_9
.LBB56_7:
	leaq	.L__unnamed_18(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$162, %esi
	callq	*%rax
.LBB56_8:
	movq	$-1, 88(%rsp)
	jmp	.LBB56_10
.LBB56_9:
	movb	7(%rsp), %al
	testb	$1, %al
	jne	.LBB56_12
	jmp	.LBB56_11
.LBB56_10:
	movq	32(%rsp), %rax
	cmpq	88(%rsp), %rax
	jbe	.LBB56_16
	jmp	.LBB56_15
.LBB56_11:
	movq	16(%rsp), %rcx
	movabsq	$9223372036854775807, %rax
	xorl	%edx, %edx
	divq	%rcx
	movq	%rax, 88(%rsp)
	jmp	.LBB56_10
.LBB56_12:
.Ltmp57:
	leaq	.L__unnamed_19(%rip), %rdi
	movq	_ZN4core9panicking11panic_const23panic_const_div_by_zero17h8f429c9723ae7081E@GOTPCREL(%rip), %rax
	callq	*%rax
.Ltmp58:
	jmp	.LBB56_14
.LBB56_13:
.Ltmp59:
	movq	_ZN4core9panicking19panic_cannot_unwind17h4ffa2eadda6e2989E@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB56_14:
	ud2
.LBB56_15:
	jmp	.LBB56_7
.LBB56_16:
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end56:
	.size	_ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E, .Lfunc_end56-_ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table56:
.Lexception9:
	.byte	255
	.byte	155
	.uleb128 .Lttbase7-.Lttbaseref7
.Lttbaseref7:
	.byte	1
	.uleb128 .Lcst_end9-.Lcst_begin9
.Lcst_begin9:
	.uleb128 .Ltmp55-.Lfunc_begin9
	.uleb128 .Ltmp56-.Ltmp55
	.uleb128 .Ltmp59-.Lfunc_begin9
	.byte	1
	.uleb128 .Ltmp56-.Lfunc_begin9
	.uleb128 .Ltmp57-.Ltmp56
	.byte	0
	.byte	0
	.uleb128 .Ltmp57-.Lfunc_begin9
	.uleb128 .Ltmp58-.Ltmp57
	.uleb128 .Ltmp59-.Lfunc_begin9
	.byte	1
	.uleb128 .Ltmp58-.Lfunc_begin9
	.uleb128 .Lfunc_end56-.Ltmp58
	.byte	0
	.byte	0
.Lcst_end9:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase7:
	.byte	0
	.p2align	2, 0x0

	.section	.text._ZN4core9panicking13assert_failed17h3576a882b3343974E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core9panicking13assert_failed17h3576a882b3343974E,@function
_ZN4core9panicking13assert_failed17h3576a882b3343974E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rcx, %r9
	movb	%dil, %al
	movq	%rsi, 8(%rsp)
	movq	%rdx, 16(%rsp)
	movq	%rsp, %rcx
	movq	%r8, (%rcx)
	movzbl	%al, %edi
	leaq	.L__unnamed_20(%rip), %r8
	movq	_ZN4core9panicking19assert_failed_inner17hafd18eee6d4029d3E@GOTPCREL(%rip), %rax
	leaq	8(%rsp), %rsi
	leaq	16(%rsp), %rcx
	movq	%r8, %rdx
	callq	*%rax
.Lfunc_end57:
	.size	_ZN4core9panicking13assert_failed17h3576a882b3343974E, .Lfunc_end57-_ZN4core9panicking13assert_failed17h3576a882b3343974E
	.cfi_endproc

	.section	.text._ZN4core9ub_checks17is_nonoverlapping7runtime17h69be12d802c2a34bE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core9ub_checks17is_nonoverlapping7runtime17h69be12d802c2a34bE,@function
_ZN4core9ub_checks17is_nonoverlapping7runtime17h69be12d802c2a34bE:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rdx, %rax
	movq	%rdi, 16(%rsp)
	movq	%rsi, 24(%rsp)
	mulq	%rcx
	movq	%rax, 32(%rsp)
	seto	%al
	andb	$1, %al
	movb	%al, 71(%rsp)
	testb	$1, 71(%rsp)
	jne	.LBB58_2
	movq	16(%rsp), %rax
	movq	24(%rsp), %rcx
	movq	32(%rsp), %rdx
	movq	%rdx, 48(%rsp)
	movq	$1, 40(%rsp)
	movq	48(%rsp), %rdx
	movq	%rdx, 8(%rsp)
	cmpq	%rcx, %rax
	jb	.LBB58_4
	jmp	.LBB58_3
.LBB58_2:
	leaq	.L__unnamed_21(%rip), %rdi
	movq	_ZN4core9panicking14panic_nounwind17h19ec6ef7ab4baa43E@GOTPCREL(%rip), %rax
	movl	$61, %esi
	callq	*%rax
.LBB58_3:
	movq	24(%rsp), %rcx
	movq	16(%rsp), %rax
	subq	%rcx, %rax
	movq	%rax, 56(%rsp)
	jmp	.LBB58_5
.LBB58_4:
	movq	16(%rsp), %rcx
	movq	24(%rsp), %rax
	subq	%rcx, %rax
	movq	%rax, 56(%rsp)
.LBB58_5:
	movq	8(%rsp), %rax
	cmpq	%rax, 56(%rsp)
	setae	%al
	andb	$1, %al
	movzbl	%al, %eax
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end58:
	.size	_ZN4core9ub_checks17is_nonoverlapping7runtime17h69be12d802c2a34bE, .Lfunc_end58-_ZN4core9ub_checks17is_nonoverlapping7runtime17h69be12d802c2a34bE
	.cfi_endproc

	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcf46670d88e87af2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcf46670d88e87af2E,@function
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcf46670d88e87af2E:
	.cfi_startproc
	xorl	%eax, %eax
	retq
.Lfunc_end59:
	.size	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcf46670d88e87af2E, .Lfunc_end59-_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcf46670d88e87af2E
	.cfi_endproc

	.section	".text._ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hcdf1fe2cf19f4a45E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hcdf1fe2cf19f4a45E,@function
_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hcdf1fe2cf19f4a45E:
.Lfunc_begin10:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception10
	subq	$120, %rsp
	.cfi_def_cfa_offset 128
	movq	%rdi, 32(%rsp)
	movq	%rsi, 40(%rsp)
	movq	%rdx, 48(%rsp)
.LBB60_1:
.Ltmp60:
	leaq	40(%rsp), %rdi
	callq	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2259e18d6fa02539E
.Ltmp61:
	movl	%eax, 28(%rsp)
	jmp	.LBB60_4
.LBB60_2:
	movq	96(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB60_3:
.Ltmp62:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 96(%rsp)
	movl	%eax, 104(%rsp)
	jmp	.LBB60_2
.LBB60_4:
	movl	28(%rsp), %eax
	movl	%eax, 60(%rsp)
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpl	$1114112, 60(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB60_6
	movq	32(%rsp), %rax
	movl	60(%rsp), %ecx
	movl	%ecx, 12(%rsp)
	movq	16(%rax), %rax
	movq	%rax, 16(%rsp)
	jmp	.LBB60_7
.LBB60_6:
	jmp	.LBB60_16
.LBB60_7:
	movq	32(%rsp), %rax
	movq	(%rax), %rax
	movq	%rax, 64(%rsp)
	movq	16(%rsp), %rax
	cmpq	64(%rsp), %rax
	je	.LBB60_10
	jmp	.LBB60_11
.LBB60_10:
.Ltmp63:
	leaq	72(%rsp), %rdi
	leaq	40(%rsp), %rsi
	callq	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h42b2953809891607E
.Ltmp64:
	jmp	.LBB60_14
.LBB60_11:
	movq	32(%rsp), %rax
	movq	16(%rsp), %rcx
	movl	12(%rsp), %esi
	movq	8(%rax), %rdx
	movl	%esi, (%rdx,%rcx,4)
	addq	$1, %rcx
	movq	%rcx, 16(%rax)
	jmp	.LBB60_1
.LBB60_12:
	jmp	.LBB60_2
.LBB60_13:
.Ltmp67:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 96(%rsp)
	movl	%eax, 104(%rsp)
	jmp	.LBB60_12
.LBB60_14:
	movq	32(%rsp), %rdi
	movq	72(%rsp), %rax
	incq	%rax
	movq	$-1, %rcx
	cmoveq	%rcx, %rax
	movq	%rax, 112(%rsp)
	movq	112(%rsp), %rsi
.Ltmp65:
	callq	_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h8c14e49934b089c1E
.Ltmp66:
	jmp	.LBB60_15
.LBB60_15:
	jmp	.LBB60_11
.LBB60_16:
	addq	$120, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end60:
	.size	_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hcdf1fe2cf19f4a45E, .Lfunc_end60-_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hcdf1fe2cf19f4a45E
	.cfi_endproc
	.section	".gcc_except_table._ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hcdf1fe2cf19f4a45E","a",@progbits
	.p2align	2, 0x0
GCC_except_table60:
.Lexception10:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end10-.Lcst_begin10
.Lcst_begin10:
	.uleb128 .Ltmp60-.Lfunc_begin10
	.uleb128 .Ltmp61-.Ltmp60
	.uleb128 .Ltmp62-.Lfunc_begin10
	.byte	0
	.uleb128 .Ltmp61-.Lfunc_begin10
	.uleb128 .Ltmp63-.Ltmp61
	.byte	0
	.byte	0
	.uleb128 .Ltmp63-.Lfunc_begin10
	.uleb128 .Ltmp66-.Ltmp63
	.uleb128 .Ltmp67-.Lfunc_begin10
	.byte	0
.Lcst_end10:
	.p2align	2, 0x0

	.section	".text._ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h8c14e49934b089c1E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h8c14e49934b089c1E,@function
_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h8c14e49934b089c1E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	16(%rdi), %rax
	movq	%rax, 24(%rsp)
	movq	8(%rsp), %rax
	movq	(%rax), %rax
	movq	%rax, 32(%rsp)
	movq	16(%rsp), %rax
	movq	24(%rsp), %rdx
	movq	32(%rsp), %rcx
	subq	%rdx, %rcx
	cmpq	%rcx, %rax
	ja	.LBB61_4
.LBB61_3:
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB61_4:
	.cfi_def_cfa_offset 48
	movq	16(%rsp), %rdx
	movq	24(%rsp), %rsi
	movq	8(%rsp), %rdi
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf586b3ecd5eb2d3cE
	jmp	.LBB61_3
.Lfunc_end61:
	.size	_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h8c14e49934b089c1E, .Lfunc_end61-_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h8c14e49934b089c1E
	.cfi_endproc

	.section	.text._ZN5alloc5alloc5alloc17h13ffbc563232da55E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc5alloc5alloc17h13ffbc563232da55E,@function
_ZN5alloc5alloc5alloc17h13ffbc563232da55E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	movq	__rust_no_alloc_shim_is_unstable@GOTPCREL(%rip), %rdi
	movl	$1, %esi
	callq	_ZN4core3ptr13read_volatile18precondition_check17h7476b258d0cecb32E
	movq	__rust_no_alloc_shim_is_unstable@GOTPCREL(%rip), %rax
	movb	(%rax), %al
	movb	%al, 39(%rsp)
	movq	16(%rsp), %rdi
	movq	8(%rsp), %rax
	movq	%rax, 24(%rsp)
	movq	24(%rsp), %rsi
	callq	*__rust_alloc@GOTPCREL(%rip)
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end62:
	.size	_ZN5alloc5alloc5alloc17h13ffbc563232da55E, .Lfunc_end62-_ZN5alloc5alloc5alloc17h13ffbc563232da55E
	.cfi_endproc

	.section	.text._ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E,@function
_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E:
	.cfi_startproc
	subq	$136, %rsp
	.cfi_def_cfa_offset 144
	movb	%cl, %al
	movb	%al, 39(%rsp)
	movq	%rsi, 48(%rsp)
	movq	%rdx, 56(%rsp)
	movq	56(%rsp), %rax
	movq	%rax, 40(%rsp)
	cmpq	$0, %rax
	jne	.LBB63_2
	leaq	48(%rsp), %rdi
	callq	_ZN4core5alloc6layout6Layout8dangling17h4092b41d461313f7E
	movq	%rax, 24(%rsp)
	jmp	.LBB63_3
.LBB63_2:
	movb	39(%rsp), %al
	testb	$1, %al
	jne	.LBB63_7
	jmp	.LBB63_6
.LBB63_3:
	movq	24(%rsp), %rdi
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	movq	24(%rsp), %rax
	movq	%rax, 64(%rsp)
	movq	$0, 72(%rsp)
.LBB63_5:
	movq	64(%rsp), %rax
	movq	72(%rsp), %rdx
	addq	$136, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB63_6:
	.cfi_def_cfa_offset 144
	movq	48(%rsp), %rdi
	movq	56(%rsp), %rsi
	callq	_ZN5alloc5alloc5alloc17h13ffbc563232da55E
	movq	%rax, 80(%rsp)
	jmp	.LBB63_8
.LBB63_7:
	movq	40(%rsp), %rdi
	movq	48(%rsp), %rcx
	movq	56(%rsp), %rax
	movq	%rcx, 88(%rsp)
	movq	%rax, 96(%rsp)
	movq	48(%rsp), %rax
	movq	%rax, 128(%rsp)
	movq	128(%rsp), %rsi
	callq	*__rust_alloc_zeroed@GOTPCREL(%rip)
	movq	%rax, 80(%rsp)
.LBB63_8:
	movq	80(%rsp), %rax
	movq	%rax, 16(%rsp)
	cmpq	$0, %rax
	jne	.LBB63_10
	movq	$0, 120(%rsp)
	movq	$0, 112(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 64(%rsp)
	movq	%rax, 72(%rsp)
	jmp	.LBB63_5
.LBB63_10:
	jmp	.LBB63_11
.LBB63_11:
	movq	16(%rsp), %rdi
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	movq	16(%rsp), %rax
	movq	%rax, 120(%rsp)
	movq	120(%rsp), %rax
	movq	%rax, 112(%rsp)
	movq	112(%rsp), %rax
	movq	%rax, 104(%rsp)
	movq	104(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rdi
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	movq	40(%rsp), %rax
	movq	8(%rsp), %rcx
	movq	%rcx, 64(%rsp)
	movq	%rax, 72(%rsp)
	jmp	.LBB63_5
.Lfunc_end63:
	.size	_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E, .Lfunc_end63-_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E
	.cfi_endproc

	.section	.text._ZN5alloc5alloc6Global9grow_impl17h37095a1597e1294cE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc5alloc6Global9grow_impl17h37095a1597e1294cE,@function
_ZN5alloc5alloc6Global9grow_impl17h37095a1597e1294cE:
	.cfi_startproc
	subq	$392, %rsp
	.cfi_def_cfa_offset 400
	movq	%rsi, 80(%rsp)
	movq	%rdi, 88(%rsp)
	movb	400(%rsp), %al
	movb	%al, 103(%rsp)
	movq	%rdx, 104(%rsp)
	movq	%rcx, 112(%rsp)
	movq	%r8, 120(%rsp)
	movq	%r9, 128(%rsp)
	movq	112(%rsp), %rax
	movq	%rax, 152(%rsp)
	cmpq	$0, 152(%rsp)
	jne	.LBB64_2
	movq	88(%rsp), %rdi
	movb	103(%rsp), %al
	movq	120(%rsp), %rsi
	movq	128(%rsp), %rdx
	movzbl	%al, %ecx
	andl	$1, %ecx
	callq	_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E
	movq	%rax, 136(%rsp)
	movq	%rdx, 144(%rsp)
	jmp	.LBB64_3
.LBB64_2:
	movq	104(%rsp), %rax
	movq	%rax, 64(%rsp)
	movq	%rax, 288(%rsp)
	movq	288(%rsp), %rax
	movq	%rax, 72(%rsp)
	movq	120(%rsp), %rcx
	movq	%rcx, 296(%rsp)
	movq	296(%rsp), %rcx
	cmpq	%rcx, %rax
	je	.LBB64_5
	jmp	.LBB64_4
.LBB64_3:
	movq	136(%rsp), %rax
	movq	144(%rsp), %rdx
	addq	$392, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB64_4:
	.cfi_def_cfa_offset 400
	movq	88(%rsp), %rdi
	movb	103(%rsp), %al
	movq	120(%rsp), %rsi
	movq	128(%rsp), %rdx
	movzbl	%al, %ecx
	andl	$1, %ecx
	callq	_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E
	movq	%rax, 272(%rsp)
	movq	%rdx, 280(%rsp)
	movq	272(%rsp), %rdx
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB64_6
	jmp	.LBB64_7
.LBB64_5:
	movq	128(%rsp), %rax
	movq	%rax, 48(%rsp)
	cmpq	152(%rsp), %rax
	setae	%al
	movb	%al, 63(%rsp)
	jmp	.LBB64_10
.LBB64_6:
	movq	272(%rsp), %rcx
	movq	280(%rsp), %rax
	movq	%rcx, 256(%rsp)
	movq	%rax, 264(%rsp)
	movq	256(%rsp), %rax
	movq	%rax, 32(%rsp)
	movq	264(%rsp), %rax
	movq	%rax, 40(%rsp)
	jmp	.LBB64_8
.LBB64_7:
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 136(%rsp)
	movq	%rax, 144(%rsp)
	jmp	.LBB64_3
.LBB64_8:
	movq	32(%rsp), %rsi
	movq	80(%rsp), %rdi
	movq	152(%rsp), %r8
	movl	$1, %ecx
	movq	%rcx, %rdx
	callq	_ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h46bc2dbafb3383c8E
	movq	32(%rsp), %rdi
	movq	80(%rsp), %rsi
	movq	152(%rsp), %rdx
	shlq	$0, %rdx
	callq	memcpy@PLT
	movq	88(%rsp), %rdi
	movq	80(%rsp), %rsi
	movq	104(%rsp), %rdx
	movq	112(%rsp), %rcx
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE
	movq	32(%rsp), %rcx
	movq	40(%rsp), %rax
	movq	%rcx, 136(%rsp)
	movq	%rax, 144(%rsp)
	jmp	.LBB64_3
.LBB64_10:
	movb	63(%rsp), %al
	movzbl	%al, %edi
	andl	$1, %edi
	callq	_ZN4core4hint16assert_unchecked18precondition_check17h37afe919f80c4ea4E
	movq	80(%rsp), %rsi
	movq	64(%rsp), %rax
	movq	72(%rsp), %rdx
	movq	48(%rsp), %rcx
	movq	%rsi, 176(%rsp)
	movq	%rsi, 304(%rsp)
	movq	304(%rsp), %rsi
	movq	%rsi, 168(%rsp)
	movq	104(%rsp), %rdi
	movq	112(%rsp), %rsi
	movq	%rdi, 184(%rsp)
	movq	%rsi, 192(%rsp)
	movq	%rcx, 200(%rsp)
	leaq	184(%rsp), %rsi
	movq	%rsi, 312(%rsp)
	leaq	184(%rsp), %rsi
	movq	%rsi, 320(%rsp)
	movq	%rax, 328(%rsp)
	movq	168(%rsp), %rdi
	movq	152(%rsp), %rsi
	callq	*__rust_realloc@GOTPCREL(%rip)
	movq	%rax, 160(%rsp)
	movq	160(%rsp), %rax
	movq	%rax, 232(%rsp)
	movq	160(%rsp), %rax
	movq	%rax, 336(%rsp)
	movq	160(%rsp), %rax
	movq	%rax, 352(%rsp)
	movq	352(%rsp), %rax
	movq	%rax, 344(%rsp)
	cmpq	$0, 344(%rsp)
	jne	.LBB64_12
	movq	$0, 224(%rsp)
	movq	$0, 216(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 136(%rsp)
	movq	%rax, 144(%rsp)
	jmp	.LBB64_3
.LBB64_12:
	jmp	.LBB64_13
.LBB64_13:
	movq	352(%rsp), %rdi
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	movb	103(%rsp), %al
	movq	160(%rsp), %rcx
	movq	%rcx, 224(%rsp)
	movq	224(%rsp), %rcx
	movq	%rcx, 216(%rsp)
	movq	216(%rsp), %rcx
	movq	%rcx, 208(%rsp)
	movq	208(%rsp), %rcx
	movq	%rcx, 24(%rsp)
	testb	$1, %al
	jne	.LBB64_16
	movq	24(%rsp), %rax
	movq	48(%rsp), %rcx
	movq	%rax, 240(%rsp)
	movq	%rcx, 248(%rsp)
	movq	%rax, 384(%rsp)
	movq	384(%rsp), %rax
	movq	%rax, 376(%rsp)
	movq	376(%rsp), %rax
	movq	%rax, 360(%rsp)
	movq	248(%rsp), %rax
	movq	%rax, 368(%rsp)
	jmp	.LBB64_18
.LBB64_16:
	movq	48(%rsp), %rax
	movq	160(%rsp), %rcx
	addq	152(%rsp), %rcx
	movq	%rcx, 8(%rsp)
	subq	152(%rsp), %rax
	movq	%rax, 16(%rsp)
	movq	8(%rsp), %rdi
	movl	$1, %esi
	callq	_ZN4core10intrinsics11write_bytes18precondition_check17h0e3014efe4c8c704E
	movq	16(%rsp), %rdx
	movq	8(%rsp), %rdi
	shlq	$0, %rdx
	xorl	%esi, %esi
	callq	memset@PLT
	movq	48(%rsp), %rcx
	movq	24(%rsp), %rax
	movq	%rax, 240(%rsp)
	movq	%rcx, 248(%rsp)
	movq	%rax, 384(%rsp)
	movq	384(%rsp), %rax
	movq	%rax, 376(%rsp)
	movq	376(%rsp), %rax
	movq	%rax, 360(%rsp)
	movq	248(%rsp), %rax
	movq	%rax, 368(%rsp)
.LBB64_18:
	movq	360(%rsp), %rdi
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	movq	360(%rsp), %rcx
	movq	368(%rsp), %rax
	movq	%rcx, 136(%rsp)
	movq	%rax, 144(%rsp)
	jmp	.LBB64_3
.Lfunc_end64:
	.size	_ZN5alloc5alloc6Global9grow_impl17h37095a1597e1294cE, .Lfunc_end64-_ZN5alloc5alloc6Global9grow_impl17h37095a1597e1294cE
	.cfi_endproc

	.section	.text._ZN5alloc7raw_vec11finish_grow17ha5e4f59160e2995fE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec11finish_grow17ha5e4f59160e2995fE,@function
_ZN5alloc7raw_vec11finish_grow17ha5e4f59160e2995fE:
	.cfi_startproc
	subq	$264, %rsp
	.cfi_def_cfa_offset 272
	movq	%r8, 40(%rsp)
	movq	%rcx, 48(%rsp)
	movq	%rdi, 56(%rsp)
	movq	%rdi, 64(%rsp)
	movq	%rsi, 72(%rsp)
	movq	%rdx, 80(%rsp)
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, 72(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB65_2
	movq	48(%rsp), %rdx
	movq	72(%rsp), %rcx
	movq	80(%rsp), %rax
	movq	%rcx, 136(%rsp)
	movq	%rax, 144(%rsp)
	movq	$0, 128(%rsp)
	movq	136(%rsp), %rcx
	movq	144(%rsp), %rax
	movq	%rcx, 112(%rsp)
	movq	%rax, 120(%rsp)
	movq	$0, 104(%rsp)
	movq	112(%rsp), %rcx
	movq	%rcx, 24(%rsp)
	movq	120(%rsp), %rax
	movq	%rax, 32(%rsp)
	movq	%rcx, 88(%rsp)
	movq	%rax, 96(%rsp)
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, 8(%rdx)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	je	.LBB65_3
	jmp	.LBB65_4
.LBB65_2:
	movq	56(%rsp), %rax
	movq	.L__unnamed_4(%rip), %rdx
	movq	.L__unnamed_4+8(%rip), %rcx
	movq	%rdx, 136(%rsp)
	movq	%rcx, 144(%rsp)
	movq	$1, 128(%rsp)
	movq	136(%rsp), %rdx
	movq	144(%rsp), %rcx
	movq	%rdx, 216(%rsp)
	movq	%rcx, 224(%rsp)
	movq	216(%rsp), %rdx
	movq	224(%rsp), %rcx
	movq	%rdx, 112(%rsp)
	movq	%rcx, 120(%rsp)
	movq	$1, 104(%rsp)
	movq	112(%rsp), %rdx
	movq	120(%rsp), %rcx
	movq	%rdx, 152(%rsp)
	movq	%rcx, 160(%rsp)
	movq	152(%rsp), %rdx
	movq	160(%rsp), %rcx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	movq	$1, (%rax)
	jmp	.LBB65_11
.LBB65_3:
	movq	24(%rsp), %rcx
	movq	48(%rsp), %rax
	movq	(%rax), %rdx
	movq	%rdx, 8(%rsp)
	movq	8(%rax), %rdx
	movq	16(%rax), %rax
	movq	%rdx, 184(%rsp)
	movq	%rax, 192(%rsp)
	movq	184(%rsp), %rax
	movq	%rax, 232(%rsp)
	movq	232(%rsp), %rax
	movq	%rcx, 240(%rsp)
	movq	240(%rsp), %rcx
	cmpq	%rcx, %rax
	sete	%al
	movb	%al, 23(%rsp)
	jmp	.LBB65_5
.LBB65_4:
	movq	32(%rsp), %rdx
	movq	24(%rsp), %rsi
	movq	40(%rsp), %rdi
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he59602fb264e04e0E
	movq	%rax, 168(%rsp)
	movq	%rdx, 176(%rsp)
	jmp	.LBB65_7
.LBB65_5:
	movb	23(%rsp), %al
	movzbl	%al, %edi
	andl	$1, %edi
	callq	_ZN4core4hint16assert_unchecked18precondition_check17h37afe919f80c4ea4E
	movq	32(%rsp), %r9
	movq	24(%rsp), %r8
	movq	8(%rsp), %rsi
	movq	40(%rsp), %rdi
	movq	184(%rsp), %rdx
	movq	192(%rsp), %rcx
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17ha7f9eb690c84ae5eE
	movq	%rax, 168(%rsp)
	movq	%rdx, 176(%rsp)
.LBB65_7:
	movq	168(%rsp), %rcx
	movq	176(%rsp), %rax
	movq	%rcx, 200(%rsp)
	movq	%rax, 208(%rsp)
	movq	200(%rsp), %rdx
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB65_9
	movq	56(%rsp), %rax
	movq	200(%rsp), %rdx
	movq	208(%rsp), %rcx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	movq	$0, (%rax)
	jmp	.LBB65_10
.LBB65_9:
	movq	56(%rsp), %rax
	movq	32(%rsp), %rcx
	movq	24(%rsp), %rdx
	movq	%rdx, 248(%rsp)
	movq	%rcx, 256(%rsp)
	movq	248(%rsp), %rdx
	movq	256(%rsp), %rcx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	movq	$1, (%rax)
.LBB65_10:
	jmp	.LBB65_11
.LBB65_11:
	movq	64(%rsp), %rax
	addq	$264, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end65:
	.size	_ZN5alloc7raw_vec11finish_grow17ha5e4f59160e2995fE, .Lfunc_end65-_ZN5alloc7raw_vec11finish_grow17ha5e4f59160e2995fE
	.cfi_endproc

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1aa3fb40e7a314aaE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1aa3fb40e7a314aaE,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1aa3fb40e7a314aaE:
	.cfi_startproc
	movq	%rsi, -48(%rsp)
	movq	%rdi, -40(%rsp)
	movq	%rdi, -32(%rsp)
	movq	-48(%rsp), %rax
	cmpq	$0, (%rax)
	jne	.LBB66_3
	jmp	.LBB66_4
.LBB66_3:
	movq	-40(%rsp), %rax
	movq	-48(%rsp), %rdx
	movq	(%rdx), %rcx
	shlq	$0, %rcx
	movq	8(%rdx), %rdx
	movq	%rdx, -24(%rsp)
	movq	$1, -16(%rsp)
	movq	%rcx, -8(%rsp)
	movq	-24(%rsp), %rcx
	movq	%rcx, (%rax)
	movq	-16(%rsp), %rcx
	movq	%rcx, 8(%rax)
	movq	-8(%rsp), %rcx
	movq	%rcx, 16(%rax)
	jmp	.LBB66_5
.LBB66_4:
	movq	-40(%rsp), %rax
	movq	$0, 8(%rax)
.LBB66_5:
	movq	-32(%rsp), %rax
	retq
.Lfunc_end66:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1aa3fb40e7a314aaE, .Lfunc_end66-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1aa3fb40e7a314aaE
	.cfi_endproc

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7903e7e608ef1670E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7903e7e608ef1670E,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7903e7e608ef1670E:
	.cfi_startproc
	movq	%rsi, -48(%rsp)
	movq	%rdi, -40(%rsp)
	movq	%rdi, -32(%rsp)
	movq	-48(%rsp), %rax
	cmpq	$0, (%rax)
	jne	.LBB67_3
	jmp	.LBB67_4
.LBB67_3:
	movq	-40(%rsp), %rax
	movq	-48(%rsp), %rdx
	imulq	$24, (%rdx), %rcx
	movq	8(%rdx), %rdx
	movq	%rdx, -24(%rsp)
	movq	$8, -16(%rsp)
	movq	%rcx, -8(%rsp)
	movq	-24(%rsp), %rcx
	movq	%rcx, (%rax)
	movq	-16(%rsp), %rcx
	movq	%rcx, 8(%rax)
	movq	-8(%rsp), %rcx
	movq	%rcx, 16(%rax)
	jmp	.LBB67_5
.LBB67_4:
	movq	-40(%rsp), %rax
	movq	$0, 8(%rax)
.LBB67_5:
	movq	-32(%rsp), %rax
	retq
.Lfunc_end67:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7903e7e608ef1670E, .Lfunc_end67-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7903e7e608ef1670E
	.cfi_endproc

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h9876064fa5d44e4fE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h9876064fa5d44e4fE,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h9876064fa5d44e4fE:
	.cfi_startproc
	movq	%rsi, -48(%rsp)
	movq	%rdi, -40(%rsp)
	movq	%rdi, -32(%rsp)
	movq	-48(%rsp), %rax
	cmpq	$0, (%rax)
	jne	.LBB68_3
	jmp	.LBB68_4
.LBB68_3:
	movq	-40(%rsp), %rax
	movq	-48(%rsp), %rdx
	movq	(%rdx), %rcx
	shlq	$2, %rcx
	movq	8(%rdx), %rdx
	movq	%rdx, -24(%rsp)
	movq	$4, -16(%rsp)
	movq	%rcx, -8(%rsp)
	movq	-24(%rsp), %rcx
	movq	%rcx, (%rax)
	movq	-16(%rsp), %rcx
	movq	%rcx, 8(%rax)
	movq	-8(%rsp), %rcx
	movq	%rcx, 16(%rax)
	jmp	.LBB68_5
.LBB68_4:
	movq	-40(%rsp), %rax
	movq	$0, 8(%rax)
.LBB68_5:
	movq	-32(%rsp), %rax
	retq
.Lfunc_end68:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h9876064fa5d44e4fE, .Lfunc_end68-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h9876064fa5d44e4fE
	.cfi_endproc

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h902838ecf84ccdc1E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h902838ecf84ccdc1E,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h902838ecf84ccdc1E:
	.cfi_startproc
	subq	$264, %rsp
	.cfi_def_cfa_offset 272
	movq	%rdi, 32(%rsp)
	movq	%rsi, 40(%rsp)
	movq	%rdx, 48(%rsp)
	movq	48(%rsp), %rcx
	movq	40(%rsp), %rax
	addq	%rcx, %rax
	movq	%rax, 24(%rsp)
	setb	%al
	andb	$1, %al
	movb	%al, 263(%rsp)
	testb	$1, 263(%rsp)
	jne	.LBB69_3
	movq	32(%rsp), %rax
	movq	24(%rsp), %rcx
	movq	%rcx, 112(%rsp)
	movq	$1, 104(%rsp)
	movq	112(%rsp), %rcx
	movq	%rcx, 96(%rsp)
	movabsq	$-9223372036854775807, %rcx
	movq	%rcx, 88(%rsp)
	movq	96(%rsp), %rcx
	movq	%rcx, 80(%rsp)
	movabsq	$-9223372036854775807, %rcx
	movq	%rcx, 72(%rsp)
	movq	80(%rsp), %rsi
	movq	(%rax), %rdi
	shlq	%rdi
	callq	_ZN4core3cmp6max_by17h9983db75b730376cE
	movq	%rax, %rsi
	movl	$4, %edi
	callq	_ZN4core3cmp6max_by17h9983db75b730376cE
	movq	%rax, %rdx
	movq	%rdx, (%rsp)
	movl	$4, %esi
	movq	%rsi, %rdi
	callq	_ZN4core5alloc6layout6Layout5array5inner17he6fccdbabe27881dE
	movq	32(%rsp), %rsi
	movq	%rax, 8(%rsp)
	movq	%rdx, 16(%rsp)
	leaq	184(%rsp), %rdi
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h9876064fa5d44e4fE
	movq	32(%rsp), %r8
	movq	8(%rsp), %rsi
	movq	16(%rsp), %rdx
	addq	$16, %r8
	leaq	160(%rsp), %rdi
	leaq	184(%rsp), %rcx
	callq	_ZN5alloc7raw_vec11finish_grow17ha5e4f59160e2995fE
	cmpq	$0, 160(%rsp)
	je	.LBB69_4
	jmp	.LBB69_5
.LBB69_3:
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 104(%rsp)
	movq	%rax, 112(%rsp)
	movq	.L__unnamed_4(%rip), %rcx
	movq	.L__unnamed_4+8(%rip), %rax
	movq	%rcx, 88(%rsp)
	movq	%rax, 96(%rsp)
	movq	88(%rsp), %rcx
	movq	96(%rsp), %rax
	movq	%rcx, 224(%rsp)
	movq	%rax, 232(%rsp)
	movq	224(%rsp), %rcx
	movq	232(%rsp), %rax
	movq	%rcx, 72(%rsp)
	movq	%rax, 80(%rsp)
	movq	72(%rsp), %rcx
	movq	80(%rsp), %rax
	movq	%rcx, 120(%rsp)
	movq	%rax, 128(%rsp)
	movq	120(%rsp), %rcx
	movq	128(%rsp), %rax
	movq	%rcx, 56(%rsp)
	movq	%rax, 64(%rsp)
	jmp	.LBB69_7
.LBB69_4:
	movq	32(%rsp), %rax
	movq	(%rsp), %rcx
	movq	168(%rsp), %rsi
	movq	176(%rsp), %rdx
	movq	%rsi, 144(%rsp)
	movq	%rdx, 152(%rsp)
	movq	$0, 136(%rsp)
	movq	144(%rsp), %rdx
	movq	%rdx, 8(%rax)
	movq	%rcx, (%rax)
	movq	.L__unnamed_22(%rip), %rcx
	movq	.L__unnamed_22+8(%rip), %rax
	movq	%rcx, 56(%rsp)
	movq	%rax, 64(%rsp)
	jmp	.LBB69_6
.LBB69_5:
	movq	168(%rsp), %rcx
	movq	176(%rsp), %rax
	movq	%rcx, 240(%rsp)
	movq	%rax, 248(%rsp)
	movq	240(%rsp), %rcx
	movq	248(%rsp), %rax
	movq	%rcx, 144(%rsp)
	movq	%rax, 152(%rsp)
	movq	$1, 136(%rsp)
	movq	144(%rsp), %rcx
	movq	152(%rsp), %rax
	movq	%rcx, 208(%rsp)
	movq	%rax, 216(%rsp)
	movq	208(%rsp), %rcx
	movq	216(%rsp), %rax
	movq	%rcx, 56(%rsp)
	movq	%rax, 64(%rsp)
	jmp	.LBB69_7
.LBB69_6:
	movq	56(%rsp), %rax
	movq	64(%rsp), %rdx
	addq	$264, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB69_7:
	.cfi_def_cfa_offset 272
	jmp	.LBB69_6
.Lfunc_end69:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h902838ecf84ccdc1E, .Lfunc_end69-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h902838ecf84ccdc1E
	.cfi_endproc

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h015f25f831395c3cE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h015f25f831395c3cE,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h015f25f831395c3cE:
.Lfunc_begin11:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception11
	subq	$200, %rsp
	.cfi_def_cfa_offset 208
	movq	%rsi, 80(%rsp)
	movq	%rdi, 88(%rsp)
	movb	%dl, %al
	movq	%rdi, 96(%rsp)
	andb	$1, %al
	movb	%al, 110(%rsp)
	movb	$1, 183(%rsp)
	movq	80(%rsp), %rax
	cmpq	$0, %rax
	jne	.LBB70_3
	movb	$0, 183(%rsp)
.Ltmp74:
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h5360dabd2c707291E
.Ltmp75:
	movq	%rdx, 64(%rsp)
	movq	%rax, 72(%rsp)
	jmp	.LBB70_19
.LBB70_3:
.Ltmp68:
	movq	80(%rsp), %rdx
	movl	$4, %esi
	movq	%rsi, %rdi
	callq	_ZN4core5alloc6layout6Layout5array5inner17he6fccdbabe27881dE
.Ltmp69:
	movq	%rdx, 48(%rsp)
	movq	%rax, 56(%rsp)
	jmp	.LBB70_6
.LBB70_4:
	testb	$1, 183(%rsp)
	jne	.LBB70_21
	jmp	.LBB70_20
.LBB70_5:
.Ltmp76:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 184(%rsp)
	movl	%eax, 192(%rsp)
	jmp	.LBB70_4
.LBB70_6:
	movq	48(%rsp), %rax
	movq	56(%rsp), %rcx
	movq	%rcx, 128(%rsp)
	movq	%rax, 136(%rsp)
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, 128(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB70_8
	movq	128(%rsp), %rcx
	movq	%rcx, 32(%rsp)
	movq	136(%rsp), %rax
	movq	%rax, 40(%rsp)
	movq	%rcx, 112(%rsp)
	movq	%rax, 120(%rsp)
	movb	110(%rsp), %al
	andb	$1, %al
	movzbl	%al, %eax
	cmpq	$0, %rax
	je	.LBB70_9
	jmp	.LBB70_10
.LBB70_8:
	movq	88(%rsp), %rax
	movq	.L__unnamed_4(%rip), %rdx
	movq	.L__unnamed_4+8(%rip), %rcx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	movq	$1, (%rax)
	jmp	.LBB70_17
.LBB70_9:
.Ltmp72:
	movq	40(%rsp), %rdx
	movq	32(%rsp), %rsi
	leaq	111(%rsp), %rdi
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he59602fb264e04e0E
.Ltmp73:
	movq	%rdx, 16(%rsp)
	movq	%rax, 24(%rsp)
	jmp	.LBB70_11
.LBB70_10:
.Ltmp70:
	movq	40(%rsp), %rdx
	movq	32(%rsp), %rsi
	leaq	111(%rsp), %rdi
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h32a115bf7cb6af50E
.Ltmp71:
	movq	%rdx, (%rsp)
	movq	%rax, 8(%rsp)
	jmp	.LBB70_13
.LBB70_11:
	movq	16(%rsp), %rax
	movq	24(%rsp), %rcx
	movq	%rcx, 144(%rsp)
	movq	%rax, 152(%rsp)
.LBB70_12:
	movq	144(%rsp), %rdx
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB70_14
	jmp	.LBB70_15
.LBB70_13:
	movq	(%rsp), %rax
	movq	8(%rsp), %rcx
	movq	%rcx, 144(%rsp)
	movq	%rax, 152(%rsp)
	jmp	.LBB70_12
.LBB70_14:
	movq	88(%rsp), %rax
	movq	80(%rsp), %rdx
	movq	144(%rsp), %rcx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	movq	$0, (%rax)
	jmp	.LBB70_16
.LBB70_15:
	movq	88(%rsp), %rax
	movq	40(%rsp), %rcx
	movq	32(%rsp), %rdx
	movq	%rdx, 160(%rsp)
	movq	%rcx, 168(%rsp)
	movq	160(%rsp), %rdx
	movq	168(%rsp), %rcx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	movq	$1, (%rax)
	jmp	.LBB70_17
.LBB70_16:
	jmp	.LBB70_18
.LBB70_17:
	jmp	.LBB70_18
.LBB70_18:
	movq	96(%rsp), %rax
	addq	$200, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB70_19:
	.cfi_def_cfa_offset 208
	movq	88(%rsp), %rax
	movq	64(%rsp), %rcx
	movq	72(%rsp), %rdx
	movq	%rdx, 8(%rax)
	movq	%rcx, 16(%rax)
	movq	$0, (%rax)
	jmp	.LBB70_16
.LBB70_20:
	movq	184(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.LBB70_21:
	jmp	.LBB70_20
.Lfunc_end70:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h015f25f831395c3cE, .Lfunc_end70-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h015f25f831395c3cE
	.cfi_endproc
	.section	".gcc_except_table._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h015f25f831395c3cE","a",@progbits
	.p2align	2, 0x0
GCC_except_table70:
.Lexception11:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end11-.Lcst_begin11
.Lcst_begin11:
	.uleb128 .Ltmp74-.Lfunc_begin11
	.uleb128 .Ltmp71-.Ltmp74
	.uleb128 .Ltmp76-.Lfunc_begin11
	.byte	0
	.uleb128 .Ltmp71-.Lfunc_begin11
	.uleb128 .Lfunc_end70-.Ltmp71
	.byte	0
	.byte	0
.Lcst_end11:
	.p2align	2, 0x0

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h5360dabd2c707291E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h5360dabd2c707291E,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h5360dabd2c707291E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	jmp	.LBB71_1
.LBB71_1:
	xorl	%eax, %eax
	movl	%eax, %edi
	addq	$4, %rdi
	callq	_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17ha2a24e272d14c505E
	xorl	%eax, %eax
	movl	$4, %edx
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end71:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h5360dabd2c707291E, .Lfunc_end71-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h5360dabd2c707291E
	.cfi_endproc

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf586b3ecd5eb2d3cE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf586b3ecd5eb2d3cE,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf586b3ecd5eb2d3cE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h902838ecf84ccdc1E
	movq	%rax, 8(%rsp)
	movq	%rdx, 16(%rsp)
	movabsq	$-9223372036854775807, %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	%rdx, 8(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB72_2
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	movq	_ZN5alloc7raw_vec12handle_error17hd343a2caeadfaacfE@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB72_2:
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end72:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf586b3ecd5eb2d3cE, .Lfunc_end72-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf586b3ecd5eb2d3cE
	.cfi_endproc

	.section	".text._ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6010daea2d011252E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6010daea2d011252E,@function
_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6010daea2d011252E:
	.cfi_startproc
	movq	%rsi, %rdx
	movq	%rdi, %rax
	retq
.Lfunc_end73:
	.size	_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6010daea2d011252E, .Lfunc_end73-_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6010daea2d011252E
	.cfi_endproc

	.section	".text._ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd6ce3481851cb66dE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd6ce3481851cb66dE,@function
_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd6ce3481851cb66dE:
	.cfi_startproc
	movq	%rsi, %rdx
	movq	%rdi, %rax
	retq
.Lfunc_end74:
	.size	_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd6ce3481851cb66dE, .Lfunc_end74-_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd6ce3481851cb66dE
	.cfi_endproc

	.section	".text._ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE,@function
_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rsi, (%rsp)
	movq	%rdx, 16(%rsp)
	movq	%rcx, 24(%rsp)
	movq	24(%rsp), %rax
	movq	%rax, 8(%rsp)
	cmpq	$0, %rax
	jne	.LBB75_2
.LBB75_1:
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB75_2:
	.cfi_def_cfa_offset 64
	movq	8(%rsp), %rsi
	movq	(%rsp), %rdi
	movq	16(%rsp), %rcx
	movq	24(%rsp), %rax
	movq	%rcx, 32(%rsp)
	movq	%rax, 40(%rsp)
	movq	16(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	48(%rsp), %rdx
	callq	*__rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB75_1
.Lfunc_end75:
	.size	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE, .Lfunc_end75-_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE
	.cfi_endproc

	.section	".text._ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h32a115bf7cb6af50E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h32a115bf7cb6af50E,@function
_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h32a115bf7cb6af50E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$1, %ecx
	callq	_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end76:
	.size	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h32a115bf7cb6af50E, .Lfunc_end76-_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h32a115bf7cb6af50E
	.cfi_endproc

	.section	".text._ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17ha7f9eb690c84ae5eE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17ha7f9eb690c84ae5eE,@function
_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17ha7f9eb690c84ae5eE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsp, %rax
	movl	$0, (%rax)
	callq	_ZN5alloc5alloc6Global9grow_impl17h37095a1597e1294cE
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end77:
	.size	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17ha7f9eb690c84ae5eE, .Lfunc_end77-_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17ha7f9eb690c84ae5eE
	.cfi_endproc

	.section	".text._ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he59602fb264e04e0E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he59602fb264e04e0E,@function
_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he59602fb264e04e0E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	xorl	%ecx, %ecx
	callq	_ZN5alloc5alloc6Global10alloc_impl17he9f624819f4630d6E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end78:
	.size	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he59602fb264e04e0E, .Lfunc_end78-_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17he59602fb264e04e0E
	.cfi_endproc

	.section	".text._ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h0c91daa765fd0177E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h0c91daa765fd0177E,@function
_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h0c91daa765fd0177E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	8(%rdi), %rax
	movq	%rax, 8(%rsp)
	movq	16(%rdi), %rax
	movq	%rax, 16(%rsp)
	movq	16(%rsp), %rcx
	movq	8(%rsp), %rdi
	movl	$1, %edx
	movq	%rdx, %rsi
	callq	_ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E
	movq	16(%rsp), %rdx
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end79:
	.size	_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h0c91daa765fd0177E, .Lfunc_end79-_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h0c91daa765fd0177E
	.cfi_endproc

	.section	".text._ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h604a1f0d2d4c45c1E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h604a1f0d2d4c45c1E,@function
_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h604a1f0d2d4c45c1E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	cmpq	$0, (%rdi)
	jne	.LBB80_2
	movq	16(%rsp), %rdi
	leaq	.L__unnamed_23(%rip), %rsi
	movl	$4, %edx
	callq	*_ZN4core3fmt9Formatter9write_str17h5c58dd14efdac51aE@GOTPCREL(%rip)
	andb	$1, %al
	movb	%al, 31(%rsp)
	jmp	.LBB80_3
.LBB80_2:
	movq	16(%rsp), %rdi
	movq	8(%rsp), %rax
	addq	$8, %rax
	movq	%rax, 32(%rsp)
	leaq	.L__unnamed_24(%rip), %rsi
	movl	$4, %edx
	leaq	32(%rsp), %rcx
	leaq	.L__unnamed_25(%rip), %r8
	callq	*_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hc8a2695f608b9b73E@GOTPCREL(%rip)
	andb	$1, %al
	movb	%al, 31(%rsp)
.LBB80_3:
	movb	31(%rsp), %al
	andb	$1, %al
	movzbl	%al, %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end80:
	.size	_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h604a1f0d2d4c45c1E, .Lfunc_end80-_ZN66_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h604a1f0d2d4c45c1E
	.cfi_endproc

	.section	".text._ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4019c5a79aa8b0f1E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4019c5a79aa8b0f1E,@function
_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4019c5a79aa8b0f1E:
	.cfi_startproc
	retq
.Lfunc_end81:
	.size	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4019c5a79aa8b0f1E, .Lfunc_end81-_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4019c5a79aa8b0f1E
	.cfi_endproc

	.section	".text._ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d9ede7d8b542cd0E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d9ede7d8b542cd0E,@function
_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d9ede7d8b542cd0E:
	.cfi_startproc
	retq
.Lfunc_end82:
	.size	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d9ede7d8b542cd0E, .Lfunc_end82-_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9d9ede7d8b542cd0E
	.cfi_endproc

	.section	".text._ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17he00a9f76f4e9dd8aE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17he00a9f76f4e9dd8aE,@function
_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17he00a9f76f4e9dd8aE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, (%rsp)
	movq	%rsi, 8(%rsp)
	cmpq	$0, (%rdi)
	jne	.LBB83_2
	movq	8(%rsp), %rax
	cmpq	$0, (%rax)
	je	.LBB83_3
	jmp	.LBB83_4
.LBB83_2:
	movq	8(%rsp), %rax
	cmpq	$0, (%rax)
	je	.LBB83_6
	jmp	.LBB83_7
.LBB83_3:
	movb	$1, 23(%rsp)
	jmp	.LBB83_5
.LBB83_4:
	movb	$0, 23(%rsp)
.LBB83_5:
	movb	23(%rsp), %al
	andb	$1, %al
	movzbl	%al, %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB83_6:
	.cfi_def_cfa_offset 32
	movb	$0, 23(%rsp)
	jmp	.LBB83_5
.LBB83_7:
	movq	8(%rsp), %rsi
	movq	(%rsp), %rdi
	addq	$8, %rdi
	addq	$8, %rsi
	callq	_ZN4core3cmp5impls56_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$usize$GT$2eq17h96fbae46dc2208feE
	andb	$1, %al
	movb	%al, 23(%rsp)
	jmp	.LBB83_5
.Lfunc_end83:
	.size	_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17he00a9f76f4e9dd8aE, .Lfunc_end83-_ZN70_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..cmp..PartialEq$GT$2eq17he00a9f76f4e9dd8aE
	.cfi_endproc

	.section	".text._ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17he734a7a5a80e5332E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17he734a7a5a80e5332E,@function
_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17he734a7a5a80e5332E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	8(%rdi), %rax
	movq	%rax, 8(%rsp)
	movq	16(%rdi), %rax
	movq	%rax, 16(%rsp)
	movq	16(%rsp), %rcx
	movq	8(%rsp), %rdi
	movl	$4, %edx
	movq	%rdx, %rsi
	callq	_ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E
	movq	16(%rsp), %rdx
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end84:
	.size	_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17he734a7a5a80e5332E, .Lfunc_end84-_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17he734a7a5a80e5332E
	.cfi_endproc

	.section	".text._ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1fe377d573639452E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1fe377d573639452E,@function
_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1fe377d573639452E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, %rsi
	movq	%rsi, 8(%rsp)
	leaq	16(%rsp), %rdi
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1aa3fb40e7a314aaE
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, 24(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB85_2
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	movq	24(%rsp), %rdx
	movq	32(%rsp), %rcx
	addq	$16, %rdi
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE
.LBB85_2:
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end85:
	.size	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1fe377d573639452E, .Lfunc_end85-_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1fe377d573639452E
	.cfi_endproc

	.section	".text._ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hac1707515fd3d6beE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hac1707515fd3d6beE,@function
_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hac1707515fd3d6beE:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, %rsi
	movq	%rsi, 8(%rsp)
	leaq	16(%rsp), %rdi
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7903e7e608ef1670E
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, 24(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB86_2
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	movq	24(%rsp), %rdx
	movq	32(%rsp), %rcx
	addq	$16, %rdi
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE
.LBB86_2:
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end86:
	.size	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hac1707515fd3d6beE, .Lfunc_end86-_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hac1707515fd3d6beE
	.cfi_endproc

	.section	".text._ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbe00bea1188b9e66E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbe00bea1188b9e66E,@function
_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbe00bea1188b9e66E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, %rsi
	movq	%rsi, 8(%rsp)
	leaq	16(%rsp), %rdi
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h9876064fa5d44e4fE
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, 24(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB87_2
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	movq	24(%rsp), %rdx
	movq	32(%rsp), %rcx
	addq	$16, %rdi
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h235d0839608a0d5dE
.LBB87_2:
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end87:
	.size	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbe00bea1188b9e66E, .Lfunc_end87-_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbe00bea1188b9e66E
	.cfi_endproc

	.section	".text._ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2259e18d6fa02539E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2259e18d6fa02539E,@function
_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2259e18d6fa02539E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	callq	_ZN4core3str11validations15next_code_point17h3c18ed530c8364b9E
	movl	%eax, 16(%rsp)
	movl	%edx, 20(%rsp)
	movl	16(%rsp), %eax
	cmpq	$0, %rax
	jne	.LBB88_2
	movl	$1114112, 12(%rsp)
	jmp	.LBB88_3
.LBB88_2:
	movl	20(%rsp), %eax
	movl	%eax, 8(%rsp)
	jmp	.LBB88_4
.LBB88_3:
	movl	12(%rsp), %eax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB88_4:
	.cfi_def_cfa_offset 32
	movl	8(%rsp), %edi
	callq	_ZN4core4char7convert18from_u32_unchecked18precondition_check17hd735cb21d94032c4E
	movl	8(%rsp), %eax
	movl	%eax, 12(%rsp)
	jmp	.LBB88_3
.Lfunc_end88:
	.size	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2259e18d6fa02539E, .Lfunc_end88-_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h2259e18d6fa02539E
	.cfi_endproc

	.section	".text._ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h42b2953809891607E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h42b2953809891607E,@function
_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h42b2953809891607E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	%rdi, 16(%rsp)
	movq	8(%rsi), %rdi
	movq	(%rsi), %rsi
	callq	_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17hfb875bf6c2b65649E
	movq	8(%rsp), %rdi
	movq	%rax, %rdx
	movq	16(%rsp), %rax
	movq	%rdx, %rcx
	addq	$3, %rcx
	shrq	$2, %rcx
	movq	%rdx, 32(%rsp)
	movq	$1, 24(%rsp)
	movq	%rcx, (%rdi)
	movq	24(%rsp), %rdx
	movq	32(%rsp), %rcx
	movq	%rdx, 8(%rdi)
	movq	%rcx, 16(%rdi)
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end89:
	.size	_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h42b2953809891607E, .Lfunc_end89-_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$9size_hint17h42b2953809891607E
	.cfi_endproc

	.section	".text._ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17haf3e26ebdf106e32E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17haf3e26ebdf106e32E,@function
_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17haf3e26ebdf106e32E:
.Lfunc_begin12:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception12
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, 24(%rsp)
	movq	24(%rsp), %rdi
	movq	%rdi, 32(%rsp)
	movq	8(%rdi), %rax
	movq	%rax, 8(%rsp)
.Ltmp77:
	callq	_ZN4core4iter6traits10exact_size17ExactSizeIterator3len17hb2f10e08cbf9afa9E
.Ltmp78:
	movq	%rax, 16(%rsp)
	jmp	.LBB90_3
.LBB90_1:
.Ltmp82:
	leaq	24(%rsp), %rdi
	callq	_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hf20ba47d8a25a9f3E
.Ltmp83:
	jmp	.LBB90_6
.LBB90_2:
.Ltmp81:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 40(%rsp)
	movl	%eax, 48(%rsp)
	jmp	.LBB90_1
.LBB90_3:
.Ltmp79:
	movq	16(%rsp), %rsi
	movq	8(%rsp), %rdi
	callq	_ZN4core3ptr57drop_in_place$LT$$u5b$std..ffi..os_str..OsString$u5d$$GT$17h162bc891fdd82777E
.Ltmp80:
	jmp	.LBB90_4
.LBB90_4:
	leaq	24(%rsp), %rdi
	callq	_ZN4core3ptr180drop_in_place$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$std..ffi..os_str..OsString$C$alloc..alloc..Global$GT$$GT$17hf20ba47d8a25a9f3E
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB90_5:
	.cfi_def_cfa_offset 64
.Ltmp84:
	movq	_ZN4core9panicking16panic_in_cleanup17he962919aa9bfab8cE@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB90_6:
	movq	40(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end90:
	.size	_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17haf3e26ebdf106e32E, .Lfunc_end90-_ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17haf3e26ebdf106e32E
	.cfi_endproc
	.section	".gcc_except_table._ZN86_$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17haf3e26ebdf106e32E","a",@progbits
	.p2align	2, 0x0
GCC_except_table90:
.Lexception12:
	.byte	255
	.byte	155
	.uleb128 .Lttbase8-.Lttbaseref8
.Lttbaseref8:
	.byte	1
	.uleb128 .Lcst_end12-.Lcst_begin12
.Lcst_begin12:
	.uleb128 .Ltmp77-.Lfunc_begin12
	.uleb128 .Ltmp78-.Ltmp77
	.uleb128 .Ltmp81-.Lfunc_begin12
	.byte	0
	.uleb128 .Ltmp82-.Lfunc_begin12
	.uleb128 .Ltmp83-.Ltmp82
	.uleb128 .Ltmp84-.Lfunc_begin12
	.byte	1
	.uleb128 .Ltmp79-.Lfunc_begin12
	.uleb128 .Ltmp80-.Ltmp79
	.uleb128 .Ltmp81-.Lfunc_begin12
	.byte	0
	.uleb128 .Ltmp80-.Lfunc_begin12
	.uleb128 .Lfunc_end90-.Ltmp80
	.byte	0
	.byte	0
.Lcst_end12:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase8:
	.byte	0
	.p2align	2, 0x0

	.section	".text._ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E,@function
_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	8(%rsp), %rax
	movq	8(%rax), %rcx
	movq	%rcx, 32(%rsp)
	movq	(%rax), %rax
	cmpq	32(%rsp), %rax
	sete	%al
	andb	$1, %al
	movb	%al, 31(%rsp)
	testb	$1, 31(%rsp)
	jne	.LBB91_4
	movq	8(%rsp), %rdi
	callq	_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h1f5047ef96ba56e1E
	movq	%rax, 16(%rsp)
	jmp	.LBB91_5
.LBB91_4:
	movq	$0, 16(%rsp)
.LBB91_5:
	movq	16(%rsp), %rax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end91:
	.size	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E, .Lfunc_end91-_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h6a95f3afb74239c3E
	.cfi_endproc

	.section	".text._ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hadce956bb19c44d6E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hadce956bb19c44d6E,@function
_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hadce956bb19c44d6E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
	movq	8(%rsp), %rax
	movq	8(%rax), %rcx
	movq	%rcx, 32(%rsp)
	movq	(%rax), %rax
	cmpq	32(%rsp), %rax
	sete	%al
	andb	$1, %al
	movb	%al, 31(%rsp)
	testb	$1, 31(%rsp)
	jne	.LBB92_4
	movq	8(%rsp), %rdi
	callq	_ZN110_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..unchecked_iterator..UncheckedIterator$GT$14next_unchecked17h47d0ffd459aad920E
	movq	%rax, 16(%rsp)
	jmp	.LBB92_5
.LBB92_4:
	movq	$0, 16(%rsp)
.LBB92_5:
	movq	16(%rsp), %rax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end92:
	.size	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hadce956bb19c44d6E, .Lfunc_end92-_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hadce956bb19c44d6E
	.cfi_endproc

	.section	".text._ZN94_$LT$$RF$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h0732cb70b58396e7E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN94_$LT$$RF$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h0732cb70b58396e7E,@function
_ZN94_$LT$$RF$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h0732cb70b58396e7E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	8(%rdi), %rax
	movq	%rax, (%rsp)
	movq	16(%rdi), %rax
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rcx
	movq	(%rsp), %rdi
	movl	$4, %edx
	movq	%rdx, %rsi
	callq	_ZN4core5slice3raw14from_raw_parts18precondition_check17h4ee0e90d380b24c5E
	jmp	.LBB93_3
.LBB93_3:
	movq	(%rsp), %rax
	movq	8(%rsp), %rcx
	shlq	$2, %rcx
	addq	%rcx, %rax
	movq	%rax, 16(%rsp)
	movq	(%rsp), %rax
	movq	16(%rsp), %rdx
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end93:
	.size	_ZN94_$LT$$RF$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h0732cb70b58396e7E, .Lfunc_end93-_ZN94_$LT$$RF$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h0732cb70b58396e7E
	.cfi_endproc

	.section	".text._ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h23d53d60e1f7f8c3E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h23d53d60e1f7f8c3E,@function
_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h23d53d60e1f7f8c3E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdx, 8(%rsp)
	movq	%rsi, %rax
	movq	8(%rsp), %rsi
	movq	%rax, 16(%rsp)
	movq	%rdi, %rax
	movq	16(%rsp), %rdi
	movq	%rax, 24(%rsp)
	movq	%rax, 32(%rsp)
	callq	_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd6ce3481851cb66dE
	movq	24(%rsp), %rdi
	movq	%rax, %rsi
	callq	_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hd705e40e22334d4cE
	movq	32(%rsp), %rax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end94:
	.size	_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h23d53d60e1f7f8c3E, .Lfunc_end94-_ZN95_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$9from_iter17h23d53d60e1f7f8c3E
	.cfi_endproc

	.section	".text._ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17ha52a72e1e0acc633E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17ha52a72e1e0acc633E,@function
_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17ha52a72e1e0acc633E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN5alloc3vec16Vec$LT$T$C$A$GT$16extend_desugared17hcdf1fe2cf19f4a45E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end95:
	.size	_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17ha52a72e1e0acc633E, .Lfunc_end95-_ZN97_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$T$C$I$GT$$GT$11spec_extend17ha52a72e1e0acc633E
	.cfi_endproc

	.section	".text._ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hd705e40e22334d4cE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hd705e40e22334d4cE,@function
_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hd705e40e22334d4cE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, %rax
	movq	%rax, (%rsp)
	callq	_ZN111_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter_nested..SpecFromIterNested$LT$T$C$I$GT$$GT$9from_iter17h67b7713e10dbee24E
	movq	(%rsp), %rax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end96:
	.size	_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hd705e40e22334d4cE, .Lfunc_end96-_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hd705e40e22334d4cE
	.cfi_endproc

	.section	.text._ZN10for_iter_111for_iter_1a17hb8a00d688e268896E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN10for_iter_111for_iter_1a17hb8a00d688e268896E,@function
_ZN10for_iter_111for_iter_1a17hb8a00d688e268896E:
	.cfi_startproc
	subq	$136, %rsp
	.cfi_def_cfa_offset 144
	callq	_ZN94_$LT$$RF$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h0732cb70b58396e7E
	movq	%rax, 8(%rsp)
	movq	%rdx, 16(%rsp)
.LBB97_1:
	leaq	8(%rsp), %rdi
	callq	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hadce956bb19c44d6E
	movq	%rax, 24(%rsp)
	movq	24(%rsp), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB97_3
	addq	$136, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB97_3:
	.cfi_def_cfa_offset 144
	movq	24(%rsp), %rax
	movq	%rax, 32(%rsp)
	leaq	32(%rsp), %rax
	movq	%rax, 120(%rsp)
	leaq	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb71c0bc844d5fd4bE(%rip), %rax
	movq	%rax, 128(%rsp)
	movq	120(%rsp), %rax
	movq	%rax, 104(%rsp)
	movq	128(%rsp), %rax
	movq	%rax, 112(%rsp)
	movq	104(%rsp), %rax
	movq	%rax, 88(%rsp)
	movq	112(%rsp), %rax
	movq	%rax, 96(%rsp)
	leaq	40(%rsp), %rdi
	leaq	.L__unnamed_26(%rip), %rsi
	movl	$2, %edx
	leaq	88(%rsp), %rcx
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments6new_v117heaacf47c44ec24ffE
	leaq	40(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17ha3dfef7ffde688d1E@GOTPCREL(%rip)
	jmp	.LBB97_1
.Lfunc_end97:
	.size	_ZN10for_iter_111for_iter_1a17hb8a00d688e268896E, .Lfunc_end97-_ZN10for_iter_111for_iter_1a17hb8a00d688e268896E
	.cfi_endproc

	.section	.text._ZN10for_iter_111for_iter_1b17h83d6b3a66f7a95d5E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN10for_iter_111for_iter_1b17h83d6b3a66f7a95d5E,@function
_ZN10for_iter_111for_iter_1b17h83d6b3a66f7a95d5E:
	.cfi_startproc
	subq	$136, %rsp
	.cfi_def_cfa_offset 144
	callq	_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17he734a7a5a80e5332E
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h122b5a9490b52eb2E
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6010daea2d011252E
	movq	%rax, 8(%rsp)
	movq	%rdx, 16(%rsp)
.LBB98_1:
	leaq	8(%rsp), %rdi
	callq	_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hadce956bb19c44d6E
	movq	%rax, 24(%rsp)
	movq	24(%rsp), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB98_3
	addq	$136, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB98_3:
	.cfi_def_cfa_offset 144
	movq	24(%rsp), %rax
	movq	%rax, 32(%rsp)
	leaq	32(%rsp), %rax
	movq	%rax, 120(%rsp)
	leaq	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb71c0bc844d5fd4bE(%rip), %rax
	movq	%rax, 128(%rsp)
	movq	120(%rsp), %rax
	movq	%rax, 104(%rsp)
	movq	128(%rsp), %rax
	movq	%rax, 112(%rsp)
	movq	104(%rsp), %rax
	movq	%rax, 88(%rsp)
	movq	112(%rsp), %rax
	movq	%rax, 96(%rsp)
	leaq	40(%rsp), %rdi
	leaq	.L__unnamed_26(%rip), %rsi
	movl	$2, %edx
	leaq	88(%rsp), %rcx
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments6new_v117heaacf47c44ec24ffE
	leaq	40(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17ha3dfef7ffde688d1E@GOTPCREL(%rip)
	jmp	.LBB98_1
.Lfunc_end98:
	.size	_ZN10for_iter_111for_iter_1b17h83d6b3a66f7a95d5E, .Lfunc_end98-_ZN10for_iter_111for_iter_1b17h83d6b3a66f7a95d5E
	.cfi_endproc

	.section	.text._ZN10for_iter_14main17hcda38913d301a47dE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN10for_iter_14main17hcda38913d301a47dE,@function
_ZN10for_iter_14main17hcda38913d301a47dE:
.Lfunc_begin13:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception13
	subq	$200, %rsp
	.cfi_def_cfa_offset 208
	movq	_ZN3std3env4args17h9a13cc635581bdb9E@GOTPCREL(%rip), %rax
	leaq	48(%rsp), %rdi
	movq	%rdi, 40(%rsp)
	callq	*%rax
	movq	40(%rsp), %rsi
.Ltmp85:
	movq	_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h94841b3384ff5350E@GOTPCREL(%rip), %rax
	leaq	80(%rsp), %rdi
	callq	*%rax
.Ltmp86:
	jmp	.LBB99_3
.LBB99_1:
.Ltmp114:
	leaq	48(%rsp), %rdi
	callq	_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7e77b91b8b0668beE
.Ltmp115:
	jmp	.LBB99_20
.LBB99_2:
.Ltmp113:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 184(%rsp)
	movl	%eax, 192(%rsp)
	jmp	.LBB99_1
.LBB99_3:
.Ltmp87:
	leaq	80(%rsp), %rdi
	callq	_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17hc4e56f9fd9e5dbc2E
.Ltmp88:
	jmp	.LBB99_4
.LBB99_4:
.Ltmp89:
	movq	_ZN73_$LT$std..env..Args$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h94841b3384ff5350E@GOTPCREL(%rip), %rax
	leaq	104(%rsp), %rdi
	leaq	48(%rsp), %rsi
	callq	*%rax
.Ltmp90:
	jmp	.LBB99_5
.LBB99_5:
	movabsq	$-9223372036854775808, %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	%rdx, 104(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB99_7
	movq	120(%rsp), %rax
	movq	%rax, 144(%rsp)
	movups	104(%rsp), %xmm0
	movaps	%xmm0, 128(%rsp)
.Ltmp93:
	leaq	128(%rsp), %rdi
	callq	_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h0c91daa765fd0177E
.Ltmp94:
	movq	%rdx, 24(%rsp)
	movq	%rax, 32(%rsp)
	jmp	.LBB99_10
.LBB99_7:
.Ltmp91:
	leaq	104(%rsp), %rdi
	callq	_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17hc4e56f9fd9e5dbc2E
.Ltmp92:
	jmp	.LBB99_19
.LBB99_8:
.Ltmp109:
	leaq	128(%rsp), %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE
.Ltmp110:
	jmp	.LBB99_1
.LBB99_9:
.Ltmp108:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 184(%rsp)
	movl	%eax, 192(%rsp)
	jmp	.LBB99_8
.LBB99_10:
.Ltmp95:
	movq	24(%rsp), %rsi
	movq	32(%rsp), %rdi
	callq	_ZN4core3str21_$LT$impl$u20$str$GT$5chars17hc60612dfa7c10438E
.Ltmp96:
	movq	%rdx, 8(%rsp)
	movq	%rax, 16(%rsp)
	jmp	.LBB99_11
.LBB99_11:
.Ltmp97:
	movq	8(%rsp), %rdx
	movq	16(%rsp), %rsi
	leaq	160(%rsp), %rdi
	callq	_ZN4core4iter6traits8iterator8Iterator7collect17h261c26de2281c052E
.Ltmp98:
	jmp	.LBB99_12
.LBB99_12:
.Ltmp99:
	leaq	160(%rsp), %rdi
	callq	_ZN10for_iter_111for_iter_1a17hb8a00d688e268896E
.Ltmp100:
	jmp	.LBB99_15
.LBB99_13:
.Ltmp104:
	leaq	160(%rsp), %rdi
	callq	_ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE
.Ltmp105:
	jmp	.LBB99_8
.LBB99_14:
.Ltmp103:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 184(%rsp)
	movl	%eax, 192(%rsp)
	jmp	.LBB99_13
.LBB99_15:
.Ltmp101:
	leaq	160(%rsp), %rdi
	callq	_ZN10for_iter_111for_iter_1b17h83d6b3a66f7a95d5E
.Ltmp102:
	jmp	.LBB99_16
.LBB99_16:
.Ltmp106:
	leaq	160(%rsp), %rdi
	callq	_ZN4core3ptr48drop_in_place$LT$alloc..vec..Vec$LT$char$GT$$GT$17h0b8555358c90530eE
.Ltmp107:
	jmp	.LBB99_17
.LBB99_17:
.Ltmp111:
	leaq	128(%rsp), %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h53cafaac85622d6cE
.Ltmp112:
	jmp	.LBB99_4
.LBB99_18:
.Ltmp116:
	movq	_ZN4core9panicking16panic_in_cleanup17he962919aa9bfab8cE@GOTPCREL(%rip), %rax
	callq	*%rax
.LBB99_19:
	leaq	48(%rsp), %rdi
	callq	_ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7e77b91b8b0668beE
	addq	$200, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB99_20:
	.cfi_def_cfa_offset 208
	movq	184(%rsp), %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end99:
	.size	_ZN10for_iter_14main17hcda38913d301a47dE, .Lfunc_end99-_ZN10for_iter_14main17hcda38913d301a47dE
	.cfi_endproc
	.section	.gcc_except_table._ZN10for_iter_14main17hcda38913d301a47dE,"a",@progbits
	.p2align	2, 0x0
GCC_except_table99:
.Lexception13:
	.byte	255
	.byte	155
	.uleb128 .Lttbase9-.Lttbaseref9
.Lttbaseref9:
	.byte	1
	.uleb128 .Lcst_end13-.Lcst_begin13
.Lcst_begin13:
	.uleb128 .Lfunc_begin13-.Lfunc_begin13
	.uleb128 .Ltmp85-.Lfunc_begin13
	.byte	0
	.byte	0
	.uleb128 .Ltmp85-.Lfunc_begin13
	.uleb128 .Ltmp86-.Ltmp85
	.uleb128 .Ltmp113-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp114-.Lfunc_begin13
	.uleb128 .Ltmp115-.Ltmp114
	.uleb128 .Ltmp116-.Lfunc_begin13
	.byte	1
	.uleb128 .Ltmp87-.Lfunc_begin13
	.uleb128 .Ltmp90-.Ltmp87
	.uleb128 .Ltmp113-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp93-.Lfunc_begin13
	.uleb128 .Ltmp94-.Ltmp93
	.uleb128 .Ltmp108-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp91-.Lfunc_begin13
	.uleb128 .Ltmp92-.Ltmp91
	.uleb128 .Ltmp113-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp109-.Lfunc_begin13
	.uleb128 .Ltmp110-.Ltmp109
	.uleb128 .Ltmp116-.Lfunc_begin13
	.byte	1
	.uleb128 .Ltmp95-.Lfunc_begin13
	.uleb128 .Ltmp98-.Ltmp95
	.uleb128 .Ltmp108-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp99-.Lfunc_begin13
	.uleb128 .Ltmp100-.Ltmp99
	.uleb128 .Ltmp103-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp104-.Lfunc_begin13
	.uleb128 .Ltmp105-.Ltmp104
	.uleb128 .Ltmp116-.Lfunc_begin13
	.byte	1
	.uleb128 .Ltmp101-.Lfunc_begin13
	.uleb128 .Ltmp102-.Ltmp101
	.uleb128 .Ltmp103-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp106-.Lfunc_begin13
	.uleb128 .Ltmp107-.Ltmp106
	.uleb128 .Ltmp108-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp111-.Lfunc_begin13
	.uleb128 .Ltmp112-.Ltmp111
	.uleb128 .Ltmp113-.Lfunc_begin13
	.byte	0
	.uleb128 .Ltmp112-.Lfunc_begin13
	.uleb128 .Lfunc_end99-.Ltmp112
	.byte	0
	.byte	0
.Lcst_end13:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase9:
	.byte	0
	.p2align	2, 0x0

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rdx
	movslq	%edi, %rsi
	leaq	_ZN10for_iter_14main17hcda38913d301a47dE(%rip), %rdi
	xorl	%ecx, %ecx
	callq	_ZN3std2rt10lang_start17h54ad9d19b72f2e73E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end100:
	.size	main, .Lfunc_end100-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_1:
	.quad	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h7a4c5a107acd5ab2E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf6abef4c3f9244d6E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hbc2a8a1c4abc22c4E
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_2,@object
	.section	.rodata..L__unnamed_2,"a",@progbits
.L__unnamed_2:
	.ascii	"unsafe precondition(s) violated: ptr::write_bytes requires that the destination pointer is aligned and non-null"
	.size	.L__unnamed_2, 111

	.type	.L__unnamed_27,@object
	.section	.rodata..L__unnamed_27,"a",@progbits
.L__unnamed_27:
	.ascii	"is_aligned_to: align is not a power-of-two"
	.size	.L__unnamed_27, 42

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_3:
	.quad	.L__unnamed_27
	.asciz	"*\000\000\000\000\000\000"
	.size	.L__unnamed_3, 16

	.type	.L__unnamed_4,@object
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	3, 0x0
.L__unnamed_4:
	.zero	8
	.zero	8
	.size	.L__unnamed_4, 16

	.type	.L__unnamed_28,@object
	.section	.rodata..L__unnamed_28,"a",@progbits
.L__unnamed_28:
	.ascii	"/rustc/8387315ab3c26a57a1f53a90f188f0bc88514bca/library/core/src/ptr/const_ptr.rs"
	.size	.L__unnamed_28, 81

	.type	.L__unnamed_5,@object
	.section	.data.rel.ro..L__unnamed_5,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_5:
	.quad	.L__unnamed_28
	.asciz	"Q\000\000\000\000\000\000\000\202\006\000\000\r\000\000"
	.size	.L__unnamed_5, 24

	.type	.L__unnamed_6,@object
	.section	.rodata..L__unnamed_6,"a",@progbits
.L__unnamed_6:
	.ascii	"unsafe precondition(s) violated: ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null and the specified memory ranges do not overlap"
	.size	.L__unnamed_6, 166

	.type	.L__unnamed_29,@object
	.section	.rodata..L__unnamed_29,"a",@progbits
.L__unnamed_29:
	.ascii	"invalid args"
	.size	.L__unnamed_29, 12

	.type	.L__unnamed_7,@object
	.section	.data.rel.ro..L__unnamed_7,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_7:
	.quad	.L__unnamed_29
	.asciz	"\f\000\000\000\000\000\000"
	.size	.L__unnamed_7, 16

	.type	.L__unnamed_30,@object
	.section	.rodata..L__unnamed_30,"a",@progbits
.L__unnamed_30:
	.ascii	"/rustc/8387315ab3c26a57a1f53a90f188f0bc88514bca/library/core/src/fmt/mod.rs"
	.size	.L__unnamed_30, 75

	.type	.L__unnamed_8,@object
	.section	.data.rel.ro..L__unnamed_8,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_8:
	.quad	.L__unnamed_30
	.asciz	"K\000\000\000\000\000\000\000a\001\000\000\r\000\000"
	.size	.L__unnamed_8, 24

	.type	.L__unnamed_9,@object
	.section	.rodata..L__unnamed_9,"a",@progbits
.L__unnamed_9:
	.ascii	"unsafe precondition(s) violated: ptr::read_volatile requires that the pointer argument is aligned and non-null"
	.size	.L__unnamed_9, 110

	.type	.L__unnamed_10,@object
	.section	.rodata..L__unnamed_10,"a",@progbits
.L__unnamed_10:
	.ascii	"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null"
	.size	.L__unnamed_10, 93

	.type	.L__unnamed_31,@object
	.section	.rodata..L__unnamed_31,"a",@progbits
.L__unnamed_31:
	.ascii	"assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize"
	.size	.L__unnamed_31, 73

	.type	.L__unnamed_32,@object
	.section	.data.rel.ro..L__unnamed_32,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_32:
	.quad	.L__unnamed_28
	.asciz	"Q\000\000\000\000\000\000\000o\003\000\000\t\000\000"
	.size	.L__unnamed_32, 24

	.type	.L__unnamed_11,@object
	.section	.rodata..L__unnamed_11,"a",@progbits
.L__unnamed_11:
	.ascii	"unsafe precondition(s) violated: ptr::sub_ptr requires `self >= origin`"
	.size	.L__unnamed_11, 71

	.type	.L__unnamed_12,@object
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	2, 0x0
.L__unnamed_12:
	.zero	4
	.zero	4
	.size	.L__unnamed_12, 8

	.type	.L__unnamed_13,@object
	.section	.rodata..L__unnamed_13,"a",@progbits
.L__unnamed_13:
	.ascii	"unsafe precondition(s) violated: invalid value for `char`"
	.size	.L__unnamed_13, 57

	.type	.L__unnamed_14,@object
	.section	.rodata..L__unnamed_14,"a",@progbits
.L__unnamed_14:
	.ascii	"unsafe precondition(s) violated: hint::assert_unchecked must never be called when the condition is false"
	.size	.L__unnamed_14, 104

	.type	.L__unnamed_15,@object
	.section	.rodata..L__unnamed_15,"a",@progbits
.L__unnamed_15:
	.ascii	"unsafe precondition(s) violated: hint::unreachable_unchecked must never be reached"
	.size	.L__unnamed_15, 82

	.type	.L__unnamed_33,@object
	.section	.rodata..L__unnamed_33,"a",@progbits
.L__unnamed_33:
	.ascii	"/rustc/8387315ab3c26a57a1f53a90f188f0bc88514bca/library/core/src/iter/traits/exact_size.rs"
	.size	.L__unnamed_33, 90

	.type	.L__unnamed_16,@object
	.section	.data.rel.ro..L__unnamed_16,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_16:
	.quad	.L__unnamed_33
	.asciz	"Z\000\000\000\000\000\000\000z\000\000\000\t\000\000"
	.size	.L__unnamed_16, 24

	.type	.L__unnamed_34,@object
	.section	.rodata..L__unnamed_34,"a",@progbits
.L__unnamed_34:
	.ascii	"/rustc/8387315ab3c26a57a1f53a90f188f0bc88514bca/library/core/src/alloc/layout.rs"
	.size	.L__unnamed_34, 80

	.type	.L__unnamed_17,@object
	.section	.data.rel.ro..L__unnamed_17,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_17:
	.quad	.L__unnamed_34
	.asciz	"P\000\000\000\000\000\000\000\303\001\000\000)\000\000"
	.size	.L__unnamed_17, 24

	.type	.L__unnamed_35,@object
	.section	.rodata..L__unnamed_35,"a",@progbits
.L__unnamed_35:
	.ascii	"/rustc/8387315ab3c26a57a1f53a90f188f0bc88514bca/library/core/src/ub_checks.rs"
	.size	.L__unnamed_35, 77

	.type	.L__unnamed_19,@object
	.section	.data.rel.ro..L__unnamed_19,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_19:
	.quad	.L__unnamed_35
	.asciz	"M\000\000\000\000\000\000\000|\000\000\0006\000\000"
	.size	.L__unnamed_19, 24

	.type	.L__unnamed_18,@object
	.section	.rodata..L__unnamed_18,"a",@progbits
.L__unnamed_18:
	.ascii	"unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`"
	.size	.L__unnamed_18, 162

	.type	.L__unnamed_20,@object
	.section	.data.rel.ro..L__unnamed_20,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_20:
	.quad	_ZN4core3ptr58drop_in_place$LT$$RF$core..option..Option$LT$usize$GT$$GT$17h596649ad4273d3e1E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h46b5eb0011ded131E
	.size	.L__unnamed_20, 32

	.type	.L__unnamed_21,@object
	.section	.rodata..L__unnamed_21,"a",@progbits
.L__unnamed_21:
	.ascii	"is_nonoverlapping: `size_of::<T>() * count` overflows a usize"
	.size	.L__unnamed_21, 61

	.type	.L__unnamed_22,@object
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	3, 0x0
.L__unnamed_22:
	.ascii	"\001\000\000\000\000\000\000\200"
	.zero	8
	.size	.L__unnamed_22, 16

	.type	.L__unnamed_23,@object
	.section	.rodata.cst4,"aM",@progbits,4
.L__unnamed_23:
	.ascii	"None"
	.size	.L__unnamed_23, 4

	.type	.L__unnamed_24,@object
.L__unnamed_24:
	.ascii	"Some"
	.size	.L__unnamed_24, 4

	.type	.L__unnamed_25,@object
	.section	.data.rel.ro..L__unnamed_25,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_25:
	.quad	_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17h2b4d751a3ac2caa0E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8212252bd2e22ba4E
	.size	.L__unnamed_25, 32

	.type	.L__unnamed_36,@object
	.section	.rodata..L__unnamed_36,"a",@progbits
.L__unnamed_36:
	.byte	10
	.size	.L__unnamed_36, 1

	.type	.L__unnamed_26,@object
	.section	.data.rel.ro..L__unnamed_26,"aw",@progbits
	.p2align	3, 0x0
.L__unnamed_26:
	.quad	1
	.zero	8
	.quad	.L__unnamed_36
	.asciz	"\001\000\000\000\000\000\000"
	.size	.L__unnamed_26, 32

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.80.0-nightly (8387315ab 2024-05-14)"
	.section	".note.GNU-stack","",@progbits
