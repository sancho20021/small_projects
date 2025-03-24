; ModuleID = 'exec_v1.56519ab262d124aa-cgu.0'
source_filename = "exec_v1.56519ab262d124aa-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { [1 x i64], ptr }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hefe80c096badb466E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf1277e1de567e1cdE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf1277e1de567e1cdE" }>, align 8
@alloc_424341b75ece3fa20496f1c69b9fb0ac = private unnamed_addr constant <{ [111 x i8] }> <{ [111 x i8] c"unsafe precondition(s) violated: ptr::write_bytes requires that the destination pointer is aligned and non-null" }>, align 1
@alloc_fad0cd83b7d1858a846a172eb260e593 = private unnamed_addr constant <{ [42 x i8] }> <{ [42 x i8] c"is_aligned_to: align is not a power-of-two" }>, align 1
@alloc_041983ee8170efdaaf95ba67fd072d26 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_fad0cd83b7d1858a846a172eb260e593, [8 x i8] c"*\00\00\00\00\00\00\00" }>, align 8
@0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_69f15bff8059880065ca4860a48f578a = private unnamed_addr constant <{ [81 x i8] }> <{ [81 x i8] c"/rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ptr/const_ptr.rs" }>, align 1
@alloc_bc25f4d4ce45194c62f9b054c79e1cf8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69f15bff8059880065ca4860a48f578a, [16 x i8] c"Q\00\00\00\00\00\00\00\EB\05\00\00\0D\00\00\00" }>, align 8
@alloc_8df0580a595a87d56789d20c7318e185 = private unnamed_addr constant <{ [166 x i8] }> <{ [166 x i8] c"unsafe precondition(s) violated: ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null and the specified memory ranges do not overlap" }>, align 1
@alloc_289b84035b3220255b84b8869a538376 = private unnamed_addr constant <{ [69 x i8] }> <{ [69 x i8] c"unsafe precondition(s) violated: usize::unchecked_add cannot overflow" }>, align 1
@alloc_ffc44ed1670ebf78d81555edceff65f6 = private unnamed_addr constant <{ [69 x i8] }> <{ [69 x i8] c"unsafe precondition(s) violated: usize::unchecked_mul cannot overflow" }>, align 1
@alloc_d4d2a2a8539eafc62756407d946babb3 = private unnamed_addr constant <{ [110 x i8] }> <{ [110 x i8] c"unsafe precondition(s) violated: ptr::read_volatile requires that the pointer argument is aligned and non-null" }>, align 1
@alloc_20b3d155afd5c58c42e598b7e6d186ef = private unnamed_addr constant <{ [93 x i8] }> <{ [93 x i8] c"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null" }>, align 1
@alloc_ab14703751a9eb3585c49b2e55e9a9e5 = private unnamed_addr constant <{ [104 x i8] }> <{ [104 x i8] c"unsafe precondition(s) violated: hint::assert_unchecked must never be called when the condition is false" }>, align 1
@alloc_cd1513ae8d1ae22acf9342b8dfa1561d = private unnamed_addr constant <{ [164 x i8] }> <{ [164 x i8] c"unsafe precondition(s) violated: Layout::from_size_align_unchecked requires that align is a power of 2 and the rounded-up allocation size does not exceed isize::MAX" }>, align 1
@1 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\00\00\00\00", [8 x i8] undef }>, align 8
@alloc_e5d730f69e25314935c1ce0c6d453b8c = private unnamed_addr constant <{ [77 x i8] }> <{ [77 x i8] c"/rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ub_checks.rs" }>, align 1
@alloc_3f55e369502ae66442d223e244fa3755 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e5d730f69e25314935c1ce0c6d453b8c, [16 x i8] c"M\00\00\00\00\00\00\00}\00\00\006\00\00\00" }>, align 8
@alloc_11195730e5236cfdc15ea13be1c301f9 = private unnamed_addr constant <{ [162 x i8] }> <{ [162 x i8] c"unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`" }>, align 1
@alloc_763310d78c99c2c1ad3f8a9821e942f3 = private unnamed_addr constant <{ [61 x i8] }> <{ [61 x i8] c"is_nonoverlapping: `size_of::<T>() * count` overflows a usize" }>, align 1
@__rust_no_alloc_shim_is_unstable = external global i8
@2 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\00\00\00\80", [8 x i8] undef }>, align 8
@alloc_a500d906b91607583596fa15e63c2ada = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"internal error: entered unreachable code" }>, align 1
@alloc_0dd2a8e6c99a57fcfe2c6f1d00915615 = private unnamed_addr constant <{ [10 x i8] }> <{ [10 x i8] c"exec_v1.rs" }>, align 1
@alloc_466e6bf9c467b28455453eef8ceb5712 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00'\00\00\00\19\00\00\00" }>, align 8
@alloc_04b0b09a3a09fd0fa014e37514eae431 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\004\00\00\00\1A\00\00\00" }>, align 8
@alloc_f480ca867be60856403fe23f61a0153e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00\83\00\00\00$\00\00\00" }>, align 8
@alloc_7e4f9bc508f529f41090b83847272315 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00\89\00\00\00\0D\00\00\00" }>, align 8
@alloc_4543aa952ce8e713eeed3b4e051eb8a8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00s\00\00\00&\00\00\00" }>, align 8
@alloc_8aa1b7189a4d5a061da3bf8da9316d47 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00\8D\00\00\00\14\00\00\00" }>, align 8
@alloc_f531d87564a7775be286359171635b59 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00\9F\00\00\00\1C\00\00\00" }>, align 8
@alloc_2701731f110eb3a8ac3cb5a242045270 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00\A3\00\00\00\13\00\00\00" }>, align 8
@alloc_afbc7bc99cc873514471c85c8b876255 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0dd2a8e6c99a57fcfe2c6f1d00915615, [16 x i8] c"\0A\00\00\00\00\00\00\00\D2\00\00\00$\00\00\00" }>, align 8
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_905721c3cf206ce86b0d110014b92e9a = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_0242e8ee118de705af76c627590b82cc = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c" " }>, align 1
@alloc_4e0023beeca5f8e5d06a41f60e7c1e6e = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr inttoptr (i64 1 to ptr), [8 x i8] zeroinitializer, ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17h8eae89d86affba27E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 {
start:
  %_8 = alloca [8 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  store ptr %main, ptr %_8, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17h4d90db0530245041E(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %v = load i64, ptr %_5, align 8
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf1277e1de567e1cdE"(ptr align 8 %_1) unnamed_addr #1 {
start:
  %self = alloca [1 x i8], align 1
  %_4 = load ptr, ptr %_1, align 8
; call std::sys::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h1fdd9f021edec46fE(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17he433adb5c67248acE"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1
  %_0 = zext i8 %_6 to i32
  ret i32 %_0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h1fdd9f021edec46fE(ptr %f) unnamed_addr #2 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h5ed64afa23be0109E(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !4
  ret void
}

; <&T as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
define internal zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hed6ec812c9f48b7dE"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8
; call core::fmt::num::<impl core::fmt::Debug for i32>::fmt
  %_0 = call zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hd52176990a0bb1f2E"(ptr align 4 %_3, ptr align 8 %f)
  ret i1 %_0
}

; <usize as core::iter::range::Step>::forward_unchecked
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17hddb5e65572d7ba20E"(i64 %start1, i64 %n) unnamed_addr #1 {
start:
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl usize>::unchecked_add::precondition_check
  call void @"_ZN4core3num23_$LT$impl$u20$usize$GT$13unchecked_add18precondition_check17h259bc686d446153cE"(i64 %start1, i64 %n) #17
  br label %bb2

bb2:                                              ; preds = %bb1
  %_0 = add nuw i64 %start1, %n
  ret i64 %_0
}

; core::intrinsics::write_bytes::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core10intrinsics11write_bytes18precondition_check17h0552f236356537ffE(ptr %addr, i64 %align) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = alloca [4 x i8], align 4
  %_8 = alloca [48 x i8], align 8
  %_6 = ptrtoint ptr %addr to i64
  %1 = icmp eq i64 %_6, 0
  br i1 %1, label %bb3, label %bb4

bb3:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %start
  %2 = call i64 @llvm.ctpop.i64(i64 %align)
  %3 = trunc i64 %2 to i32
  store i32 %3, ptr %0, align 4
  %_9 = load i32, ptr %0, align 4
  %4 = icmp eq i32 %_9, 1
  br i1 %4, label %bb5, label %bb6

bb2:                                              ; preds = %bb5, %bb3
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_424341b75ece3fa20496f1c69b9fb0ac, i64 111) #18
  unreachable

bb5:                                              ; preds = %bb4
  %_13 = sub i64 %align, 1
  %_12 = and i64 %_6, %_13
  %_3 = icmp eq i64 %_12, 0
  br i1 %_3, label %bb1, label %bb2

bb6:                                              ; preds = %bb4
  store ptr @alloc_041983ee8170efdaaf95ba67fd072d26, ptr %_8, align 8
  %5 = getelementptr inbounds i8, ptr %_8, i64 8
  store i64 1, ptr %5, align 8
  %6 = load ptr, ptr @0, align 8
  %7 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %8 = getelementptr inbounds i8, ptr %_8, i64 32
  store ptr %6, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %_8, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %10, i64 8
  store i64 0, ptr %11, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h3eea515d05f7a35eE(ptr align 8 %_8, ptr align 8 @alloc_bc25f4d4ce45194c62f9b054c79e1cf8) #19
          to label %unreachable unwind label %terminate

bb1:                                              ; preds = %bb5
  ret void

terminate:                                        ; preds = %bb6
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hea865182d7ce50afE() #20
  unreachable

unreachable:                                      ; preds = %bb6
  unreachable
}

; core::intrinsics::copy_nonoverlapping::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core10intrinsics19copy_nonoverlapping18precondition_check17hbcc74fdbdf0ffda5E(ptr %src, ptr %dst, i64 %size, i64 %align, i64 %count) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = alloca [4 x i8], align 4
  %1 = alloca [4 x i8], align 4
  %_23 = alloca [48 x i8], align 8
  %_14 = alloca [48 x i8], align 8
  %_12 = ptrtoint ptr %src to i64
  %2 = icmp eq i64 %_12, 0
  br i1 %2, label %bb8, label %bb9

bb8:                                              ; preds = %start
  br label %bb6

bb9:                                              ; preds = %start
  %3 = call i64 @llvm.ctpop.i64(i64 %align)
  %4 = trunc i64 %3 to i32
  store i32 %4, ptr %1, align 4
  %_15 = load i32, ptr %1, align 4
  %5 = icmp eq i32 %_15, 1
  br i1 %5, label %bb10, label %bb11

bb6:                                              ; preds = %bb10, %bb8
  br label %bb7

bb10:                                             ; preds = %bb9
  %_19 = sub i64 %align, 1
  %_18 = and i64 %_12, %_19
  %_6 = icmp eq i64 %_18, 0
  br i1 %_6, label %bb1, label %bb6

bb11:                                             ; preds = %bb9
  store ptr @alloc_041983ee8170efdaaf95ba67fd072d26, ptr %_14, align 8
  %6 = getelementptr inbounds i8, ptr %_14, i64 8
  store i64 1, ptr %6, align 8
  %7 = load ptr, ptr @0, align 8
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %9 = getelementptr inbounds i8, ptr %_14, i64 32
  store ptr %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  store i64 %8, ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %_14, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %11, i64 8
  store i64 0, ptr %12, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h3eea515d05f7a35eE(ptr align 8 %_14, ptr align 8 @alloc_bc25f4d4ce45194c62f9b054c79e1cf8) #19
          to label %unreachable unwind label %terminate

bb1:                                              ; preds = %bb10
  %_21 = ptrtoint ptr %dst to i64
  %13 = icmp eq i64 %_21, 0
  br i1 %13, label %bb13, label %bb14

bb7:                                              ; preds = %bb4, %bb5, %bb6
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_8df0580a595a87d56789d20c7318e185, i64 166) #18
  unreachable

bb13:                                             ; preds = %bb1
  br label %bb5

bb14:                                             ; preds = %bb1
  %14 = call i64 @llvm.ctpop.i64(i64 %align)
  %15 = trunc i64 %14 to i32
  store i32 %15, ptr %0, align 4
  %_24 = load i32, ptr %0, align 4
  %16 = icmp eq i32 %_24, 1
  br i1 %16, label %bb15, label %bb16

bb5:                                              ; preds = %bb15, %bb13
  br label %bb7

bb15:                                             ; preds = %bb14
  %_28 = sub i64 %align, 1
  %_27 = and i64 %_21, %_28
  %_7 = icmp eq i64 %_27, 0
  br i1 %_7, label %bb2, label %bb5

bb16:                                             ; preds = %bb14
  store ptr @alloc_041983ee8170efdaaf95ba67fd072d26, ptr %_23, align 8
  %17 = getelementptr inbounds i8, ptr %_23, i64 8
  store i64 1, ptr %17, align 8
  %18 = load ptr, ptr @0, align 8
  %19 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %20 = getelementptr inbounds i8, ptr %_23, i64 32
  store ptr %18, ptr %20, align 8
  %21 = getelementptr inbounds i8, ptr %20, i64 8
  store i64 %19, ptr %21, align 8
  %22 = getelementptr inbounds i8, ptr %_23, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %22, align 8
  %23 = getelementptr inbounds i8, ptr %22, i64 8
  store i64 0, ptr %23, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h3eea515d05f7a35eE(ptr align 8 %_23, ptr align 8 @alloc_bc25f4d4ce45194c62f9b054c79e1cf8) #19
          to label %unreachable unwind label %terminate

bb2:                                              ; preds = %bb15
; invoke core::ub_checks::is_nonoverlapping::runtime
  %_9 = invoke zeroext i1 @_ZN4core9ub_checks17is_nonoverlapping7runtime17h3a2d06a617370af9E(ptr %src, ptr %dst, i64 %size, i64 %count)
          to label %bb18 unwind label %terminate

terminate:                                        ; preds = %bb11, %bb16, %bb2
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %25 = extractvalue { ptr, i32 } %24, 0
  %26 = extractvalue { ptr, i32 } %24, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hea865182d7ce50afE() #20
  unreachable

bb18:                                             ; preds = %bb2
  br i1 %_9, label %bb3, label %bb4

bb4:                                              ; preds = %bb18
  br label %bb7

bb3:                                              ; preds = %bb18
  ret void

unreachable:                                      ; preds = %bb11, %bb16
  unreachable
}

; core::intrinsics::unlikely
; Function Attrs: nounwind nonlazybind uwtable
define internal zeroext i1 @_ZN4core10intrinsics8unlikely17hf407b208f27b8e6bE(i1 zeroext %b) unnamed_addr #4 {
start:
  ret i1 %b
}

; core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17h0b044679db068a46E"(ptr align 8 %self, ptr align 8 %other) unnamed_addr #1 {
start:
  %_3 = load i64, ptr %self, align 8
  %_4 = load i64, ptr %other, align 8
  %0 = icmp ugt i64 %_3, %_4
  %1 = zext i1 %0 to i8
  %2 = icmp ult i64 %_3, %_4
  %3 = zext i1 %2 to i8
  %_0 = sub nsw i8 %1, %3
  ret i8 %_0
}

; core::cmp::max_by
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @_ZN4core3cmp6max_by17hadf1ac163bd5bd80E(i64 %0, i64 %1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %2 = alloca [16 x i8], align 8
  %_9 = alloca [1 x i8], align 1
  %_4 = alloca [1 x i8], align 1
  %_0 = alloca [8 x i8], align 8
  %v2 = alloca [8 x i8], align 8
  %v1 = alloca [8 x i8], align 8
  store i64 %0, ptr %v1, align 8
  store i64 %1, ptr %v2, align 8
  store i8 1, ptr %_9, align 1
; invoke core::ops::function::FnOnce::call_once
  %3 = invoke i8 @_ZN4core3ops8function6FnOnce9call_once17hd181c9491c2d8e48E(ptr align 8 %v1, ptr align 8 %v2)
          to label %bb1 unwind label %cleanup

bb6:                                              ; preds = %cleanup
  br label %bb10

cleanup:                                          ; preds = %start
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = extractvalue { ptr, i32 } %4, 0
  %6 = extractvalue { ptr, i32 } %4, 1
  store ptr %5, ptr %2, align 8
  %7 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %6, ptr %7, align 8
  br label %bb6

bb1:                                              ; preds = %start
  store i8 %3, ptr %_4, align 1
  %_8 = load i8, ptr %_4, align 1
  switch i8 %_8, label %bb2 [
    i8 -1, label %bb4
    i8 0, label %bb4
    i8 1, label %bb3
  ]

bb2:                                              ; preds = %bb1
  unreachable

bb4:                                              ; preds = %bb1, %bb1
  %8 = load i64, ptr %v2, align 8
  store i64 %8, ptr %_0, align 8
  %9 = load i8, ptr %_9, align 1
  %10 = trunc i8 %9 to i1
  br i1 %10, label %bb8, label %bb5

bb3:                                              ; preds = %bb1
  store i8 0, ptr %_9, align 1
  %11 = load i64, ptr %v1, align 8
  store i64 %11, ptr %_0, align 8
  br label %bb5

bb5:                                              ; preds = %bb3, %bb8, %bb4
  %12 = load i64, ptr %_0, align 8
  ret i64 %12

bb8:                                              ; preds = %bb4
  br label %bb5

bb10:                                             ; preds = %bb6
  %13 = load i8, ptr %_9, align 1
  %14 = trunc i8 %13 to i1
  br i1 %14, label %bb9, label %bb7

bb7:                                              ; preds = %bb9, %bb10
  %15 = load ptr, ptr %2, align 8
  %16 = getelementptr inbounds i8, ptr %2, i64 8
  %17 = load i32, ptr %16, align 8
  %18 = insertvalue { ptr, i32 } poison, ptr %15, 0
  %19 = insertvalue { ptr, i32 } %18, i32 %17, 1
  resume { ptr, i32 } %19

bb9:                                              ; preds = %bb10
  br label %bb7
}

; core::fmt::num::<impl core::fmt::Debug for i32>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hd52176990a0bb1f2E"(ptr align 4 %self, ptr align 8 %f) unnamed_addr #1 {
start:
  %_0 = alloca [1 x i8], align 1
  %0 = getelementptr inbounds i8, ptr %f, i64 52
  %_4 = load i32, ptr %0, align 4
  %_3 = and i32 %_4, 16
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %2 = getelementptr inbounds i8, ptr %f, i64 52
  %_6 = load i32, ptr %2, align 4
  %_5 = and i32 %_6, 32
  %3 = icmp eq i32 %_5, 0
  br i1 %3, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
  %4 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h5c1cc86791bc1522E"(ptr align 4 %self, ptr align 8 %f)
  %5 = zext i1 %4 to i8
  store i8 %5, ptr %_0, align 1
  br label %bb6

bb4:                                              ; preds = %bb2
; call core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
  %6 = call zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he44de31354db2158E"(ptr align 4 %self, ptr align 8 %f)
  %7 = zext i1 %6 to i8
  store i8 %7, ptr %_0, align 1
  br label %bb5

bb3:                                              ; preds = %bb2
; call core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
  %8 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17ha980ac9704f347d8E"(ptr align 4 %self, ptr align 8 %f)
  %9 = zext i1 %8 to i8
  store i8 %9, ptr %_0, align 1
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  br label %bb6

bb6:                                              ; preds = %bb1, %bb5
  %10 = load i8, ptr %_0, align 1
  %11 = trunc i8 %10 to i1
  ret i1 %11
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hbad76aae779359c6E(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces, ptr align 8 %args) unnamed_addr #1 {
start:
  store ptr %pieces, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 2, ptr %0, align 8
  %1 = load ptr, ptr @0, align 8
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 32
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 16
  store ptr %args, ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 1, ptr %6, align 8
  ret void
}

; core::fmt::Arguments::new_const
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments9new_const17h8de8285d1e415f0fE(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces) unnamed_addr #1 {
start:
  store ptr %pieces, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 1, ptr %0, align 8
  %1 = load ptr, ptr @0, align 8
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 32
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 0, ptr %6, align 8
  ret void
}

; core::num::<impl usize>::unchecked_add::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @"_ZN4core3num23_$LT$impl$u20$usize$GT$13unchecked_add18precondition_check17h259bc686d446153cE"(i64 %lhs, i64 %rhs) unnamed_addr #3 {
start:
  %0 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %lhs, i64 %rhs)
  %_6.0 = extractvalue { i64, i1 } %0, 0
  %_6.1 = extractvalue { i64, i1 } %0, 1
  br i1 %_6.1, label %bb1, label %bb2

bb2:                                              ; preds = %start
  ret void

bb1:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_289b84035b3220255b84b8869a538376, i64 69) #18
  unreachable
}

; core::num::<impl usize>::unchecked_mul::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @"_ZN4core3num23_$LT$impl$u20$usize$GT$13unchecked_mul18precondition_check17hefe36670947257c6E"(i64 %lhs, i64 %rhs) unnamed_addr #3 {
start:
  %0 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %lhs, i64 %rhs)
  %_6.0 = extractvalue { i64, i1 } %0, 0
  %_6.1 = extractvalue { i64, i1 } %0, 1
  br i1 %_6.1, label %bb1, label %bb2

bb2:                                              ; preds = %start
  ret void

bb1:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_ffc44ed1670ebf78d81555edceff65f6, i64 69) #18
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hefe80c096badb466E"(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  %0 = load ptr, ptr %_1, align 8
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h4c85095f8975599dE(ptr %0)
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h4c85095f8975599dE(ptr %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf1277e1de567e1cdE"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8
  %3 = getelementptr inbounds i8, ptr %1, i64 8
  %4 = load i32, ptr %3, align 8
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1
  resume { ptr, i32 } %6

cleanup:                                          ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  store ptr %8, ptr %1, align 8
  %10 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h5ed64afa23be0109E(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  call void %_1()
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @_ZN4core3ops8function6FnOnce9call_once17h77772a5118839ae9E() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [0 x i8], align 1
; invoke exec_v1::Array<T>::borrow::{{closure}}
  %_0 = invoke zeroext i1 @"_ZN7exec_v114Array$LT$T$GT$6borrow28_$u7b$$u7b$closure$u7d$$u7d$17h52d181637889b400E"(ptr align 1 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %1 = load ptr, ptr %0, align 8
  %2 = getelementptr inbounds i8, ptr %0, i64 8
  %3 = load i32, ptr %2, align 8
  %4 = insertvalue { ptr, i32 } poison, ptr %1, 0
  %5 = insertvalue { ptr, i32 } %4, i32 %3, 1
  resume { ptr, i32 } %5

cleanup:                                          ; preds = %start
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  store ptr %7, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %8, ptr %9, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i1 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h7a1baf619c78755eE() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [0 x i8], align 1
; invoke exec_v1::Array<T>::new::{{closure}}
  invoke void @"_ZN7exec_v114Array$LT$T$GT$3new28_$u7b$$u7b$closure$u7d$$u7d$17h0e76fbff48ea8035E"(ptr align 1 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %1 = load ptr, ptr %0, align 8
  %2 = getelementptr inbounds i8, ptr %0, i64 8
  %3 = load i32, ptr %2, align 8
  %4 = insertvalue { ptr, i32 } poison, ptr %1, 0
  %5 = insertvalue { ptr, i32 } %4, i32 %3, 1
  resume { ptr, i32 } %5

cleanup:                                          ; preds = %start
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  store ptr %7, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %8, ptr %9, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 1 ptr @_ZN4core3ops8function6FnOnce9call_once17hd0e7d3d9f3b24899E() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [0 x i8], align 1
; invoke exec_v1::Borrow<T>::replace::{{closure}}
  %_0 = invoke align 1 ptr @"_ZN7exec_v115Borrow$LT$T$GT$7replace28_$u7b$$u7b$closure$u7d$$u7d$17hdc71cccb12095375E"(ptr align 1 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %1 = load ptr, ptr %0, align 8
  %2 = getelementptr inbounds i8, ptr %0, i64 8
  %3 = load i32, ptr %2, align 8
  %4 = insertvalue { ptr, i32 } poison, ptr %1, 0
  %5 = insertvalue { ptr, i32 } %4, i32 %3, 1
  resume { ptr, i32 } %5

cleanup:                                          ; preds = %start
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  store ptr %7, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %8, ptr %9, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret ptr %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @_ZN4core3ops8function6FnOnce9call_once17hd181c9491c2d8e48E(ptr align 8 %0, ptr align 8 %1) unnamed_addr #1 {
start:
  %_2 = alloca [16 x i8], align 8
  store ptr %0, ptr %_2, align 8
  %2 = getelementptr inbounds i8, ptr %_2, i64 8
  store ptr %1, ptr %2, align 8
  %3 = load ptr, ptr %_2, align 8
  %4 = getelementptr inbounds i8, ptr %_2, i64 8
  %5 = load ptr, ptr %4, align 8
; call core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
  %_0 = call i8 @"_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17h0b044679db068a46E"(ptr align 8 %3, ptr align 8 %5)
  ret i8 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 1 ptr @_ZN4core3ops8function6FnOnce9call_once17hfda4b49ef8349476E() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [0 x i8], align 1
; invoke exec_v1::Borrow<T>::read::{{closure}}
  %_0 = invoke align 1 ptr @"_ZN7exec_v115Borrow$LT$T$GT$4read28_$u7b$$u7b$closure$u7d$$u7d$17h55faaa3ca3f2bb34E"(ptr align 1 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %1 = load ptr, ptr %0, align 8
  %2 = getelementptr inbounds i8, ptr %0, i64 8
  %3 = load i32, ptr %2, align 8
  %4 = insertvalue { ptr, i32 } poison, ptr %1, 0
  %5 = insertvalue { ptr, i32 } %4, i32 %3, 1
  resume { ptr, i32 } %5

cleanup:                                          ; preds = %start
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  store ptr %7, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %8, ptr %9, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret ptr %_0
}

; core::ptr::read_volatile::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core3ptr13read_volatile18precondition_check17hd83978e3bb478068E(ptr %addr, i64 %align) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = alloca [4 x i8], align 4
  %_8 = alloca [48 x i8], align 8
  %_6 = ptrtoint ptr %addr to i64
  %1 = icmp eq i64 %_6, 0
  br i1 %1, label %bb3, label %bb4

bb3:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %start
  %2 = call i64 @llvm.ctpop.i64(i64 %align)
  %3 = trunc i64 %2 to i32
  store i32 %3, ptr %0, align 4
  %_9 = load i32, ptr %0, align 4
  %4 = icmp eq i32 %_9, 1
  br i1 %4, label %bb5, label %bb6

bb2:                                              ; preds = %bb5, %bb3
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_d4d2a2a8539eafc62756407d946babb3, i64 110) #18
  unreachable

bb5:                                              ; preds = %bb4
  %_13 = sub i64 %align, 1
  %_12 = and i64 %_6, %_13
  %_3 = icmp eq i64 %_12, 0
  br i1 %_3, label %bb1, label %bb2

bb6:                                              ; preds = %bb4
  store ptr @alloc_041983ee8170efdaaf95ba67fd072d26, ptr %_8, align 8
  %5 = getelementptr inbounds i8, ptr %_8, i64 8
  store i64 1, ptr %5, align 8
  %6 = load ptr, ptr @0, align 8
  %7 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %8 = getelementptr inbounds i8, ptr %_8, i64 32
  store ptr %6, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %_8, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %10, i64 8
  store i64 0, ptr %11, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h3eea515d05f7a35eE(ptr align 8 %_8, ptr align 8 @alloc_bc25f4d4ce45194c62f9b054c79e1cf8) #19
          to label %unreachable unwind label %terminate

bb1:                                              ; preds = %bb5
  ret void

terminate:                                        ; preds = %bb6
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hea865182d7ce50afE() #20
  unreachable

unreachable:                                      ; preds = %bb6
  unreachable
}

; core::ptr::drop_in_place<exec_v1::Array<i32>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$exec_v1..Array$LT$i32$GT$$GT$17hb37936d2bb4c5872E"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<vstd::simple_pptr::PPtr<i32>>>
  call void @"_ZN4core3ptr78drop_in_place$LT$alloc..vec..Vec$LT$vstd..simple_pptr..PPtr$LT$i32$GT$$GT$$GT$17h3990e745265f84d6E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<i32>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$i32$GT$$GT$17hbed7f08e1a0bbd00E"(ptr align 8 %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb2bb135c24939ec8E"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<i32>>
  invoke void @"_ZN4core3ptr54drop_in_place$LT$alloc..raw_vec..RawVec$LT$i32$GT$$GT$17h1fd113af733cc596E"(ptr align 8 %_1) #21
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<i32>>
  call void @"_ZN4core3ptr54drop_in_place$LT$alloc..raw_vec..RawVec$LT$i32$GT$$GT$17h1fd113af733cc596E"(ptr align 8 %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17hb6fcb0ed7ad330b7E() #20
  unreachable

bb1:                                              ; preds = %bb3
  %8 = load ptr, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  %10 = load i32, ptr %9, align 8
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1
  resume { ptr, i32 } %12
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<i32>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr54drop_in_place$LT$alloc..raw_vec..RawVec$LT$i32$GT$$GT$17h1fd113af733cc596E"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfae5577759daab49E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<vstd::simple_pptr::PPtr<i32>>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr78drop_in_place$LT$alloc..vec..Vec$LT$vstd..simple_pptr..PPtr$LT$i32$GT$$GT$$GT$17h3990e745265f84d6E"(ptr align 8 %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9a592e2466e71906E"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<vstd::simple_pptr::PPtr<i32>>>
  invoke void @"_ZN4core3ptr85drop_in_place$LT$alloc..raw_vec..RawVec$LT$vstd..simple_pptr..PPtr$LT$i32$GT$$GT$$GT$17h9e4a5a3c1163d722E"(ptr align 8 %_1) #21
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<vstd::simple_pptr::PPtr<i32>>>
  call void @"_ZN4core3ptr85drop_in_place$LT$alloc..raw_vec..RawVec$LT$vstd..simple_pptr..PPtr$LT$i32$GT$$GT$$GT$17h9e4a5a3c1163d722E"(ptr align 8 %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17hb6fcb0ed7ad330b7E() #20
  unreachable

bb1:                                              ; preds = %bb3
  %8 = load ptr, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  %10 = load i32, ptr %9, align 8
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1
  resume { ptr, i32 } %12
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<vstd::simple_pptr::PPtr<i32>>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$alloc..raw_vec..RawVec$LT$vstd..simple_pptr..PPtr$LT$i32$GT$$GT$$GT$17h9e4a5a3c1163d722E"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8e3ad2006b99dd9eE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h91148c5309966ad3E"(ptr align 8 %_1) unnamed_addr #1 {
start:
  ret void
}

; core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %ptr) unnamed_addr #3 {
start:
  %_4 = ptrtoint ptr %ptr to i64
  %0 = icmp eq i64 %_4, 0
  br i1 %0, label %bb1, label %bb2

bb1:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_20b3d155afd5c58c42e598b7e6d186ef, i64 93) #18
  unreachable

bb2:                                              ; preds = %start
  ret void
}

; core::hint::assert_unchecked::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core4hint16assert_unchecked18precondition_check17h286af9399513f8f3E(i1 zeroext %cond) unnamed_addr #3 {
start:
  br i1 %cond, label %bb2, label %bb1

bb1:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_ab14703751a9eb3585c49b2e55e9a9e5, i64 104) #18
  unreachable

bb2:                                              ; preds = %start
  ret void
}

; core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h18636b3d4ab8910bE"(ptr align 8 %self) unnamed_addr #1 {
start:
; call <core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next
  %0 = call { i64, i64 } @"_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17hc3170b232b7c34a2E"(ptr align 8 %self)
  %_0.0 = extractvalue { i64, i64 } %0, 0
  %_0.1 = extractvalue { i64, i64 } %0, 1
  %1 = insertvalue { i64, i64 } poison, i64 %_0.0, 0
  %2 = insertvalue { i64, i64 } %1, i64 %_0.1, 1
  ret { i64, i64 } %2
}

; core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hfd805233854ca994E(i64 %size, i64 %align) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
; invoke core::alloc::layout::Layout::is_size_align_valid
  %_3 = invoke zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h27157fff07002cf3E(i64 %size, i64 %align)
          to label %bb1 unwind label %terminate

terminate:                                        ; preds = %start
  %0 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %1 = extractvalue { ptr, i32 } %0, 0
  %2 = extractvalue { ptr, i32 } %0, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hea865182d7ce50afE() #20
  unreachable

bb1:                                              ; preds = %start
  br i1 %_3, label %bb2, label %bb3

bb3:                                              ; preds = %bb1
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_cd1513ae8d1ae22acf9342b8dfa1561d, i64 164) #18
  unreachable

bb2:                                              ; preds = %bb1
  ret void
}

; core::alloc::layout::Layout::repeat
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core5alloc6layout6Layout6repeat17h26c4997c792ae0f3E(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %self, i64 %n) unnamed_addr #1 {
start:
  %0 = alloca [1 x i8], align 1
  %_16 = alloca [8 x i8], align 8
  %_14 = alloca [24 x i8], align 8
  %self3 = alloca [16 x i8], align 8
  %_11 = alloca [16 x i8], align 8
  %self2 = alloca [16 x i8], align 8
  %self1 = alloca [16 x i8], align 8
  %_7 = alloca [16 x i8], align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 8
  %len = load i64, ptr %1, align 8
  %self4 = load i64, ptr %self, align 8
  store i64 %self4, ptr %_16, align 8
  %_17 = load i64, ptr %_16, align 8
  %_18 = icmp uge i64 %_17, 1
  %_19 = icmp ule i64 %_17, -9223372036854775808
  %_20 = and i1 %_18, %_19
  %self5 = add i64 %len, %_17
  %_22 = sub i64 %self5, 1
  %_25 = sub i64 %_17, 1
  %_24 = xor i64 %_25, -1
  %len_rounded_up = and i64 %_22, %_24
  %_5 = sub i64 %len_rounded_up, %len
  %padded_size = add i64 %len, %_5
  %2 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %padded_size, i64 %n)
  %_29.0 = extractvalue { i64, i1 } %2, 0
  %_29.1 = extractvalue { i64, i1 } %2, 1
  %3 = call i1 @llvm.expect.i1(i1 %_29.1, i1 false)
  %4 = zext i1 %3 to i8
  store i8 %4, ptr %0, align 1
  %5 = load i8, ptr %0, align 1
  %_26 = trunc i8 %5 to i1
  br i1 %_26, label %bb4, label %bb5

bb5:                                              ; preds = %start
  %6 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %_29.0, ptr %6, align 8
  store i64 1, ptr %self2, align 8
  %7 = getelementptr inbounds i8, ptr %self2, i64 8
  %v = load i64, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %v, ptr %8, align 8
  store i64 0, ptr %self1, align 8
  %9 = getelementptr inbounds i8, ptr %self1, i64 8
  %v6 = load i64, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %_7, i64 8
  store i64 %v6, ptr %10, align 8
  store i64 0, ptr %_7, align 8
  %11 = getelementptr inbounds i8, ptr %_7, i64 8
  %alloc_size = load i64, ptr %11, align 8
  %_36 = sub i64 9223372036854775807, %_25
  %_35 = icmp ugt i64 %alloc_size, %_36
  br i1 %_35, label %bb6, label %bb7

bb4:                                              ; preds = %start
  %12 = load i64, ptr @0, align 8
  %13 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %12, ptr %self2, align 8
  %14 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %13, ptr %14, align 8
  %15 = load i64, ptr @1, align 8
  %16 = load i64, ptr getelementptr inbounds (i8, ptr @1, i64 8), align 8
  store i64 %15, ptr %self1, align 8
  %17 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %16, ptr %17, align 8
  store i64 0, ptr %_0, align 8
  br label %bb1

bb7:                                              ; preds = %bb5
  store i64 %self4, ptr %self3, align 8
  %18 = getelementptr inbounds i8, ptr %self3, i64 8
  store i64 %alloc_size, ptr %18, align 8
  %v.0 = load i64, ptr %self3, align 8
  %19 = getelementptr inbounds i8, ptr %self3, i64 8
  %v.1 = load i64, ptr %19, align 8
  store i64 %v.0, ptr %_11, align 8
  %20 = getelementptr inbounds i8, ptr %_11, i64 8
  store i64 %v.1, ptr %20, align 8
  %layout.0 = load i64, ptr %_11, align 8
  %21 = getelementptr inbounds i8, ptr %_11, i64 8
  %layout.1 = load i64, ptr %21, align 8
  store i64 %layout.0, ptr %_14, align 8
  %22 = getelementptr inbounds i8, ptr %_14, i64 8
  store i64 %layout.1, ptr %22, align 8
  %23 = getelementptr inbounds i8, ptr %_14, i64 16
  store i64 %padded_size, ptr %23, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_14, i64 24, i1 false)
  br label %bb2

bb6:                                              ; preds = %bb5
  %24 = load i64, ptr @0, align 8
  %25 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %24, ptr %self3, align 8
  %26 = getelementptr inbounds i8, ptr %self3, i64 8
  store i64 %25, ptr %26, align 8
  store i64 0, ptr %_0, align 8
  br label %bb1

bb2:                                              ; preds = %bb1, %bb7
  ret void

bb1:                                              ; preds = %bb4, %bb6
  br label %bb2
}

; core::slice::raw::from_raw_parts::precondition_check
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal void @_ZN4core5slice3raw14from_raw_parts18precondition_check17h3aa1340e63dfc8edE(ptr %data, i64 %size, i64 %align, i64 %len) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = alloca [4 x i8], align 4
  %max_len = alloca [8 x i8], align 8
  %_12 = alloca [48 x i8], align 8
  %_10 = ptrtoint ptr %data to i64
  %1 = icmp eq i64 %_10, 0
  br i1 %1, label %bb6, label %bb7

bb6:                                              ; preds = %start
  br label %bb4

bb7:                                              ; preds = %start
  %2 = call i64 @llvm.ctpop.i64(i64 %align)
  %3 = trunc i64 %2 to i32
  store i32 %3, ptr %0, align 4
  %_13 = load i32, ptr %0, align 4
  %4 = icmp eq i32 %_13, 1
  br i1 %4, label %bb8, label %bb9

bb4:                                              ; preds = %bb8, %bb6
  br label %bb5

bb8:                                              ; preds = %bb7
  %_17 = sub i64 %align, 1
  %_16 = and i64 %_10, %_17
  %_5 = icmp eq i64 %_16, 0
  br i1 %_5, label %bb1, label %bb4

bb9:                                              ; preds = %bb7
  store ptr @alloc_041983ee8170efdaaf95ba67fd072d26, ptr %_12, align 8
  %5 = getelementptr inbounds i8, ptr %_12, i64 8
  store i64 1, ptr %5, align 8
  %6 = load ptr, ptr @0, align 8
  %7 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %8 = getelementptr inbounds i8, ptr %_12, i64 32
  store ptr %6, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %_12, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %10, i64 8
  store i64 0, ptr %11, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h3eea515d05f7a35eE(ptr align 8 %_12, ptr align 8 @alloc_bc25f4d4ce45194c62f9b054c79e1cf8) #19
          to label %unreachable unwind label %terminate

bb1:                                              ; preds = %bb8
  %_19 = icmp eq i64 %size, 0
  %12 = icmp eq i64 %size, 0
  br i1 %12, label %bb11, label %bb12

bb5:                                              ; preds = %bb3, %bb4
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_11195730e5236cfdc15ea13be1c301f9, i64 162) #18
  unreachable

bb11:                                             ; preds = %bb1
  store i64 -1, ptr %max_len, align 8
  br label %bb14

bb12:                                             ; preds = %bb1
  br i1 %_19, label %panic, label %bb13

bb14:                                             ; preds = %bb13, %bb11
  %_20 = load i64, ptr %max_len, align 8
  %_7 = icmp ule i64 %len, %_20
  br i1 %_7, label %bb2, label %bb3

bb13:                                             ; preds = %bb12
  %13 = udiv i64 9223372036854775807, %size
  store i64 %13, ptr %max_len, align 8
  br label %bb14

panic:                                            ; preds = %bb12
; invoke core::panicking::panic_const::panic_const_div_by_zero
  invoke void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h29d33a10d2cc93f0E(ptr align 8 @alloc_3f55e369502ae66442d223e244fa3755) #19
          to label %unreachable unwind label %terminate

terminate:                                        ; preds = %bb9, %panic
  %14 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %15 = extractvalue { ptr, i32 } %14, 0
  %16 = extractvalue { ptr, i32 } %14, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17hea865182d7ce50afE() #20
  unreachable

unreachable:                                      ; preds = %bb9, %panic
  unreachable

bb3:                                              ; preds = %bb14
  br label %bb5

bb2:                                              ; preds = %bb14
  ret void
}

; core::ub_checks::is_nonoverlapping::runtime
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @_ZN4core9ub_checks17is_nonoverlapping7runtime17h3a2d06a617370af9E(ptr %src, ptr %dst, i64 %size, i64 %count) unnamed_addr #1 {
start:
  %0 = alloca [1 x i8], align 1
  %diff = alloca [8 x i8], align 8
  %_9 = alloca [16 x i8], align 8
  %src_usize = ptrtoint ptr %src to i64
  %dst_usize = ptrtoint ptr %dst to i64
  %1 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %size, i64 %count)
  %_15.0 = extractvalue { i64, i1 } %1, 0
  %_15.1 = extractvalue { i64, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_15.1, i1 false)
  %3 = zext i1 %2 to i8
  store i8 %3, ptr %0, align 1
  %4 = load i8, ptr %0, align 1
  %_12 = trunc i8 %4 to i1
  br i1 %_12, label %bb2, label %bb3

bb3:                                              ; preds = %start
  %5 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 %_15.0, ptr %5, align 8
  store i64 1, ptr %_9, align 8
  %6 = getelementptr inbounds i8, ptr %_9, i64 8
  %size1 = load i64, ptr %6, align 8
  %_22 = icmp ult i64 %src_usize, %dst_usize
  br i1 %_22, label %bb4, label %bb5

bb2:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1 @alloc_763310d78c99c2c1ad3f8a9821e942f3, i64 61) #18
  unreachable

bb5:                                              ; preds = %bb3
  %7 = sub i64 %src_usize, %dst_usize
  store i64 %7, ptr %diff, align 8
  br label %bb6

bb4:                                              ; preds = %bb3
  %8 = sub i64 %dst_usize, %src_usize
  store i64 %8, ptr %diff, align 8
  br label %bb6

bb6:                                              ; preds = %bb4, %bb5
  %_11 = load i64, ptr %diff, align 8
  %_0 = icmp uge i64 %_11, %size1
  ret i1 %_0
}

; vstd::simple_pptr::PPtr<V>::new
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN4vstd11simple_pptr13PPtr$LT$V$GT$3new17h9a00104cb8ea0e9bE"(i32 %v) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_13 = alloca [8 x i8], align 8
  br label %bb4

bb4:                                              ; preds = %start
; invoke vstd::raw_ptr::allocate
  %_6 = invoke ptr @_ZN4vstd7raw_ptr8allocate17haf43c87912e14848E(i64 4, i64 4)
          to label %bb5 unwind label %cleanup

bb6:                                              ; No predecessors!
  store i64 4, ptr %_13, align 8
  br label %bb3

bb3:                                              ; preds = %bb5, %bb6
  %p = load i64, ptr %_13, align 8
  %_12 = inttoptr i64 %p to ptr
  store i32 %v, ptr %_12, align 4
  ret i64 %p

bb2:                                              ; preds = %cleanup
  %1 = load ptr, ptr %0, align 8
  %2 = getelementptr inbounds i8, ptr %0, i64 8
  %3 = load i32, ptr %2, align 8
  %4 = insertvalue { ptr, i32 } poison, ptr %1, 0
  %5 = insertvalue { ptr, i32 } %4, i32 %3, 1
  resume { ptr, i32 } %5

cleanup:                                          ; preds = %bb4
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  store ptr %7, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %8, ptr %9, align 8
  br label %bb2

bb5:                                              ; preds = %bb4
  %_11 = ptrtoint ptr %_6 to i64
  store i64 %_11, ptr %_13, align 8
  br label %bb3
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17he433adb5c67248acE"() unnamed_addr #1 {
start:
  ret i8 0
}

; alloc::vec::Vec<T>::new
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc3vec12Vec$LT$T$GT$3new17h1c991167e67589d4E"(ptr sret([24 x i8]) align 8 %_0) unnamed_addr #1 {
start:
  store i64 0, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr inttoptr (i64 4 to ptr), ptr %0, align 8
  %1 = getelementptr inbounds i8, ptr %_0, i64 16
  store i64 0, ptr %1, align 8
  ret void
}

; alloc::vec::Vec<T>::new
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc3vec12Vec$LT$T$GT$3new17h45015e188983f172E"(ptr sret([24 x i8]) align 8 %_0) unnamed_addr #1 {
start:
  store i64 0, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr inttoptr (i64 8 to ptr), ptr %0, align 8
  %1 = getelementptr inbounds i8, ptr %_0, i64 16
  store i64 0, ptr %1, align 8
  ret void
}

; alloc::vec::Vec<T,A>::len
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17hb2a5a87e59cc4172E"(ptr align 8 %self) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 16
  %_0 = load i64, ptr %0, align 8
  ret i64 %_0
}

; alloc::vec::Vec<T,A>::len
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17hb47e093ab52f167eE"(ptr align 8 %self) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 16
  %_0 = load i64, ptr %0, align 8
  ret i64 %_0
}

; alloc::vec::Vec<T,A>::pop
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i32, i32 } @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3pop17h300ede68afa62c2aE"(ptr align 8 %self) unnamed_addr #1 {
start:
  %_5 = alloca [8 x i8], align 8
  %_0 = alloca [8 x i8], align 4
  %0 = getelementptr inbounds i8, ptr %self, i64 16
  %_2 = load i64, ptr %0, align 8
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %start
  store i32 0, ptr %_0, align 4
  br label %bb3

bb2:                                              ; preds = %start
  %2 = getelementptr inbounds i8, ptr %self, i64 16
  %3 = getelementptr inbounds i8, ptr %self, i64 16
  %4 = load i64, ptr %3, align 8
  %5 = sub i64 %4, 1
  store i64 %5, ptr %2, align 8
  %6 = getelementptr inbounds i8, ptr %self, i64 16
  %_4 = load i64, ptr %6, align 8
  br label %bb6

bb3:                                              ; preds = %bb8, %bb1
  %7 = load i32, ptr %_0, align 4
  %8 = getelementptr inbounds i8, ptr %_0, i64 4
  %9 = load i32, ptr %8, align 4
  %10 = insertvalue { i32, i32 } poison, i32 %7, 0
  %11 = insertvalue { i32, i32 } %10, i32 %9, 1
  ret { i32, i32 } %11

bb6:                                              ; preds = %bb2
  %12 = load i64, ptr %self, align 8
  store i64 %12, ptr %_5, align 8
  br label %bb4

bb5:                                              ; No predecessors!
  store i64 -1, ptr %_5, align 8
  br label %bb4

bb4:                                              ; preds = %bb6, %bb5
  %13 = load i64, ptr %_5, align 8
  %cond = icmp ult i64 %_4, %13
  br label %bb7

bb7:                                              ; preds = %bb4
; call core::hint::assert_unchecked::precondition_check
  call void @_ZN4core4hint16assert_unchecked18precondition_check17h286af9399513f8f3E(i1 zeroext %cond) #17
  br label %bb8

bb8:                                              ; preds = %bb7
  %14 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %14, align 8
  %15 = getelementptr inbounds i8, ptr %self, i64 16
  %count = load i64, ptr %15, align 8
  %src = getelementptr inbounds i32, ptr %self1, i64 %count
  %_6 = load i32, ptr %src, align 4
  %16 = getelementptr inbounds i8, ptr %_0, i64 4
  store i32 %_6, ptr %16, align 4
  store i32 1, ptr %_0, align 4
  br label %bb3
}

; alloc::vec::Vec<T,A>::push
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17hc005cf2537d82a12E"(ptr align 8 %self, i64 %value) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8
  br label %bb9

bb9:                                              ; preds = %start
  %2 = load i64, ptr %self, align 8
  store i64 %2, ptr %_5, align 8
  br label %bb7

bb8:                                              ; No predecessors!
  store i64 -1, ptr %_5, align 8
  br label %bb7

bb7:                                              ; preds = %bb9, %bb8
  %3 = load i64, ptr %_5, align 8
  %_4 = icmp eq i64 %len, %3
  br i1 %_4, label %bb1, label %bb3

bb3:                                              ; preds = %bb7
  br label %bb4

bb1:                                              ; preds = %bb7
; invoke alloc::raw_vec::RawVec<T,A>::grow_one
  invoke void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h75d06ceed2b17565E"(ptr align 8 %self)
          to label %bb2 unwind label %cleanup

bb4:                                              ; preds = %bb2, %bb3
  %4 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %4, align 8
  %end = getelementptr inbounds i64, ptr %self1, i64 %len
  store i64 %value, ptr %end, align 8
  %5 = getelementptr inbounds i8, ptr %self, i64 16
  %6 = add i64 %len, 1
  store i64 %6, ptr %5, align 8
  ret void

bb6:                                              ; preds = %cleanup
  %7 = load ptr, ptr %0, align 8
  %8 = getelementptr inbounds i8, ptr %0, i64 8
  %9 = load i32, ptr %8, align 8
  %10 = insertvalue { ptr, i32 } poison, ptr %7, 0
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1
  resume { ptr, i32 } %11

cleanup:                                          ; preds = %bb1
  %12 = landingpad { ptr, i32 }
          cleanup
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
  store ptr %13, ptr %0, align 8
  %15 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %14, ptr %15, align 8
  br label %bb6

bb2:                                              ; preds = %bb1
  br label %bb4
}

; alloc::vec::Vec<T,A>::push
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17hc9dca40045e66215E"(ptr align 8 %self, i32 %value) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8
  br label %bb9

bb9:                                              ; preds = %start
  %2 = load i64, ptr %self, align 8
  store i64 %2, ptr %_5, align 8
  br label %bb7

bb8:                                              ; No predecessors!
  store i64 -1, ptr %_5, align 8
  br label %bb7

bb7:                                              ; preds = %bb9, %bb8
  %3 = load i64, ptr %_5, align 8
  %_4 = icmp eq i64 %len, %3
  br i1 %_4, label %bb1, label %bb3

bb3:                                              ; preds = %bb7
  br label %bb4

bb1:                                              ; preds = %bb7
; invoke alloc::raw_vec::RawVec<T,A>::grow_one
  invoke void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h7f186cc5654d5cf9E"(ptr align 8 %self)
          to label %bb2 unwind label %cleanup

bb4:                                              ; preds = %bb2, %bb3
  %4 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %4, align 8
  %end = getelementptr inbounds i32, ptr %self1, i64 %len
  store i32 %value, ptr %end, align 4
  %5 = getelementptr inbounds i8, ptr %self, i64 16
  %6 = add i64 %len, 1
  store i64 %6, ptr %5, align 8
  ret void

bb6:                                              ; preds = %cleanup
  %7 = load ptr, ptr %0, align 8
  %8 = getelementptr inbounds i8, ptr %0, i64 8
  %9 = load i32, ptr %8, align 8
  %10 = insertvalue { ptr, i32 } poison, ptr %7, 0
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1
  resume { ptr, i32 } %11

cleanup:                                          ; preds = %bb1
  %12 = landingpad { ptr, i32 }
          cleanup
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
  store ptr %13, ptr %0, align 8
  %15 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %14, ptr %15, align 8
  br label %bb6

bb2:                                              ; preds = %bb1
  br label %bb4
}

; alloc::alloc::exchange_malloc
; Function Attrs: inlinehint nonlazybind uwtable
define internal ptr @_ZN5alloc5alloc15exchange_malloc17h1f290aac71a702e4E(i64 %size, i64 %align) unnamed_addr #1 {
start:
  %_4 = alloca [16 x i8], align 8
  br label %bb4

bb4:                                              ; preds = %start
; call core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
  call void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hfd805233854ca994E(i64 %size, i64 %align) #17
  br label %bb5

bb5:                                              ; preds = %bb4
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h6f3bfdc26261c320E(ptr align 1 inttoptr (i64 1 to ptr), i64 %align, i64 %size, i1 zeroext false)
  %1 = extractvalue { ptr, i64 } %0, 0
  %2 = extractvalue { ptr, i64 } %0, 1
  store ptr %1, ptr %_4, align 8
  %3 = getelementptr inbounds i8, ptr %_4, i64 8
  store i64 %2, ptr %3, align 8
  %4 = load ptr, ptr %_4, align 8
  %5 = ptrtoint ptr %4 to i64
  %6 = icmp eq i64 %5, 0
  %_5 = select i1 %6, i64 1, i64 0
  %7 = icmp eq i64 %_5, 0
  br i1 %7, label %bb3, label %bb2

bb3:                                              ; preds = %bb5
  %ptr.0 = load ptr, ptr %_4, align 8
  %8 = getelementptr inbounds i8, ptr %_4, i64 8
  %ptr.1 = load i64, ptr %8, align 8
  ret ptr %ptr.0

bb2:                                              ; preds = %bb5
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17h6235a660a5e8e3a6E(i64 %align, i64 %size) #19
  unreachable

bb1:                                              ; No predecessors!
  unreachable
}

; alloc::alloc::alloc
; Function Attrs: inlinehint nonlazybind uwtable
define internal ptr @_ZN5alloc5alloc5alloc17h2d1a48a5f5bb0fd6E(i64 %0, i64 %1) unnamed_addr #1 {
start:
  %2 = alloca [1 x i8], align 1
  %_11 = alloca [8 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %3, align 8
  br label %bb3

bb3:                                              ; preds = %start
; call core::ptr::read_volatile::precondition_check
  call void @_ZN4core3ptr13read_volatile18precondition_check17hd83978e3bb478068E(ptr @__rust_no_alloc_shim_is_unstable, i64 1) #17
  br label %bb5

bb5:                                              ; preds = %bb3
  %4 = load volatile i8, ptr @__rust_no_alloc_shim_is_unstable, align 1
  store i8 %4, ptr %2, align 1
  %_2 = load i8, ptr %2, align 1
  %5 = getelementptr inbounds i8, ptr %layout, i64 8
  %_3 = load i64, ptr %5, align 8
  %self = load i64, ptr %layout, align 8
  store i64 %self, ptr %_11, align 8
  %_12 = load i64, ptr %_11, align 8
  %_13 = icmp uge i64 %_12, 1
  %_14 = icmp ule i64 %_12, -9223372036854775808
  %_15 = and i1 %_13, %_14
  %_0 = call ptr @__rust_alloc(i64 %_3, i64 %_12) #17
  ret ptr %_0
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h6f3bfdc26261c320E(ptr align 1 %self, i64 %0, i64 %1, i1 zeroext %zeroed) unnamed_addr #1 {
start:
  %_38 = alloca [8 x i8], align 8
  %data4 = alloca [8 x i8], align 8
  %ptr = alloca [16 x i8], align 8
  %_28 = alloca [8 x i8], align 8
  %_20 = alloca [8 x i8], align 8
  %self3 = alloca [8 x i8], align 8
  %self2 = alloca [8 x i8], align 8
  %_11 = alloca [8 x i8], align 8
  %layout1 = alloca [16 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %data = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %size = load i64, ptr %3, align 8
  %4 = icmp eq i64 %size, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %self5 = load i64, ptr %layout, align 8
  store i64 %self5, ptr %_20, align 8
  %_21 = load i64, ptr %_20, align 8
  %_22 = icmp uge i64 %_21, 1
  %_23 = icmp ule i64 %_21, -9223372036854775808
  %_24 = and i1 %_22, %_23
  %ptr6 = getelementptr i8, ptr null, i64 %_21
  br label %bb7

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb7:                                              ; preds = %bb2
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %ptr6) #17
  store ptr %ptr6, ptr %_28, align 8
  %5 = load ptr, ptr %_28, align 8
  store ptr %5, ptr %data, align 8
  store ptr %ptr6, ptr %data4, align 8
  store ptr %ptr6, ptr %ptr, align 8
  %6 = getelementptr inbounds i8, ptr %ptr, i64 8
  store i64 0, ptr %6, align 8
  br label %bb10

bb9:                                              ; No predecessors!
  unreachable

bb10:                                             ; preds = %bb7
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %ptr6) #17
  br label %bb12

bb12:                                             ; preds = %bb10
  %_33.0 = load ptr, ptr %ptr, align 8
  %7 = getelementptr inbounds i8, ptr %ptr, i64 8
  %_33.1 = load i64, ptr %7, align 8
  store ptr %_33.0, ptr %_0, align 8
  %8 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %_33.1, ptr %8, align 8
  br label %bb6

bb6:                                              ; preds = %bb21, %bb14, %bb12
  %9 = load ptr, ptr %_0, align 8
  %10 = getelementptr inbounds i8, ptr %_0, i64 8
  %11 = load i64, ptr %10, align 8
  %12 = insertvalue { ptr, i64 } poison, ptr %9, 0
  %13 = insertvalue { ptr, i64 } %12, i64 %11, 1
  ret { ptr, i64 } %13

bb4:                                              ; preds = %bb1
  %14 = load i64, ptr %layout, align 8
  %15 = getelementptr inbounds i8, ptr %layout, i64 8
  %16 = load i64, ptr %15, align 8
; call alloc::alloc::alloc
  %17 = call ptr @_ZN5alloc5alloc5alloc17h2d1a48a5f5bb0fd6E(i64 %14, i64 %16)
  store ptr %17, ptr %raw_ptr, align 8
  br label %bb5

bb3:                                              ; preds = %bb1
  %18 = load i64, ptr %layout, align 8
  %19 = getelementptr inbounds i8, ptr %layout, i64 8
  %20 = load i64, ptr %19, align 8
  store i64 %18, ptr %layout1, align 8
  %21 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %20, ptr %21, align 8
  %self7 = load i64, ptr %layout, align 8
  store i64 %self7, ptr %_38, align 8
  %_39 = load i64, ptr %_38, align 8
  %_40 = icmp uge i64 %_39, 1
  %_41 = icmp ule i64 %_39, -9223372036854775808
  %_42 = and i1 %_40, %_41
  %22 = call ptr @__rust_alloc_zeroed(i64 %size, i64 %_39) #17
  store ptr %22, ptr %raw_ptr, align 8
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %ptr8 = load ptr, ptr %raw_ptr, align 8
  %_44 = ptrtoint ptr %ptr8 to i64
  %23 = icmp eq i64 %_44, 0
  br i1 %23, label %bb14, label %bb15

bb14:                                             ; preds = %bb5
  store ptr null, ptr %self3, align 8
  store ptr null, ptr %self2, align 8
  %24 = load ptr, ptr @0, align 8
  %25 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %24, ptr %_0, align 8
  %26 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %25, ptr %26, align 8
  br label %bb6

bb15:                                             ; preds = %bb5
  br label %bb16

bb16:                                             ; preds = %bb15
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %ptr8) #17
  br label %bb18

bb18:                                             ; preds = %bb16
  store ptr %ptr8, ptr %self3, align 8
  %v = load ptr, ptr %self3, align 8
  store ptr %v, ptr %self2, align 8
  %v9 = load ptr, ptr %self2, align 8
  store ptr %v9, ptr %_11, align 8
  %ptr10 = load ptr, ptr %_11, align 8
  br label %bb19

bb19:                                             ; preds = %bb18
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %ptr10) #17
  br label %bb21

bb21:                                             ; preds = %bb19
  store ptr %ptr10, ptr %_0, align 8
  %27 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %size, ptr %27, align 8
  br label %bb6
}

; alloc::alloc::Global::grow_impl
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global9grow_impl17hc4ab2c2a56faffa1E(ptr align 1 %self, ptr %ptr, i64 %0, i64 %1, i64 %2, i64 %3, i1 zeroext %zeroed) unnamed_addr #1 {
start:
  %layout14 = alloca [16 x i8], align 8
  %_77 = alloca [8 x i8], align 8
  %data13 = alloca [8 x i8], align 8
  %ptr12 = alloca [16 x i8], align 8
  %_65 = alloca [8 x i8], align 8
  %ptr11 = alloca [8 x i8], align 8
  %self10 = alloca [8 x i8], align 8
  %self9 = alloca [8 x i8], align 8
  %self8 = alloca [8 x i8], align 8
  %_59 = alloca [8 x i8], align 8
  %_52 = alloca [8 x i8], align 8
  %_46 = alloca [8 x i8], align 8
  %layout7 = alloca [16 x i8], align 8
  %self6 = alloca [16 x i8], align 8
  %_37 = alloca [16 x i8], align 8
  %len = alloca [8 x i8], align 8
  %data = alloca [8 x i8], align 8
  %ptr5 = alloca [8 x i8], align 8
  %self4 = alloca [8 x i8], align 8
  %self3 = alloca [8 x i8], align 8
  %_25 = alloca [8 x i8], align 8
  %new_size = alloca [8 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %self2 = alloca [8 x i8], align 8
  %ptr1 = alloca [8 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %old_size = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %new_layout = alloca [16 x i8], align 8
  %old_layout = alloca [16 x i8], align 8
  store i64 %0, ptr %old_layout, align 8
  %4 = getelementptr inbounds i8, ptr %old_layout, i64 8
  store i64 %1, ptr %4, align 8
  store i64 %2, ptr %new_layout, align 8
  %5 = getelementptr inbounds i8, ptr %new_layout, i64 8
  store i64 %3, ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %7 = load i64, ptr %6, align 8
  store i64 %7, ptr %old_size, align 8
  %8 = load i64, ptr %old_size, align 8
  %9 = icmp eq i64 %8, 0
  br i1 %9, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %10 = load i64, ptr %new_layout, align 8
  %11 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %12 = load i64, ptr %11, align 8
; call alloc::alloc::Global::alloc_impl
  %13 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h6f3bfdc26261c320E(ptr align 1 %self, i64 %10, i64 %12, i1 zeroext %zeroed)
  %14 = extractvalue { ptr, i64 } %13, 0
  %15 = extractvalue { ptr, i64 } %13, 1
  store ptr %14, ptr %_0, align 8
  %16 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %15, ptr %16, align 8
  br label %bb11

bb1:                                              ; preds = %start
  %self15 = load i64, ptr %old_layout, align 8
  store i64 %self15, ptr %_46, align 8
  %_47 = load i64, ptr %_46, align 8
  %_48 = icmp uge i64 %_47, 1
  %_49 = icmp ule i64 %_47, -9223372036854775808
  %_50 = and i1 %_48, %_49
  %self16 = load i64, ptr %new_layout, align 8
  store i64 %self16, ptr %_52, align 8
  %_53 = load i64, ptr %_52, align 8
  %_54 = icmp uge i64 %_53, 1
  %_55 = icmp ule i64 %_53, -9223372036854775808
  %_56 = and i1 %_54, %_55
  %_11 = icmp eq i64 %_47, %_53
  br i1 %_11, label %bb3, label %bb4

bb11:                                             ; preds = %bb24, %bb32, %bb10, %bb40, %bb2
  %17 = load ptr, ptr %_0, align 8
  %18 = getelementptr inbounds i8, ptr %_0, i64 8
  %19 = load i64, ptr %18, align 8
  %20 = insertvalue { ptr, i64 } poison, ptr %17, 0
  %21 = insertvalue { ptr, i64 } %20, i64 %19, 1
  ret { ptr, i64 } %21

bb4:                                              ; preds = %bb1
  %22 = load i64, ptr %new_layout, align 8
  %23 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %24 = load i64, ptr %23, align 8
; call alloc::alloc::Global::alloc_impl
  %25 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h6f3bfdc26261c320E(ptr align 1 %self, i64 %22, i64 %24, i1 zeroext %zeroed)
  %26 = extractvalue { ptr, i64 } %25, 0
  %27 = extractvalue { ptr, i64 } %25, 1
  store ptr %26, ptr %self6, align 8
  %28 = getelementptr inbounds i8, ptr %self6, i64 8
  store i64 %27, ptr %28, align 8
  %29 = load ptr, ptr %self6, align 8
  %30 = ptrtoint ptr %29 to i64
  %31 = icmp eq i64 %30, 0
  %_81 = select i1 %31, i64 1, i64 0
  %32 = icmp eq i64 %_81, 0
  br i1 %32, label %bb35, label %bb34

bb3:                                              ; preds = %bb1
  %33 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %new_size17 = load i64, ptr %33, align 8
  %34 = load i64, ptr %old_size, align 8
  %cond = icmp uge i64 %new_size17, %34
  br label %bb12

bb35:                                             ; preds = %bb4
  %v.0 = load ptr, ptr %self6, align 8
  %35 = getelementptr inbounds i8, ptr %self6, i64 8
  %v.1 = load i64, ptr %35, align 8
  store ptr %v.0, ptr %_37, align 8
  %36 = getelementptr inbounds i8, ptr %_37, i64 8
  store i64 %v.1, ptr %36, align 8
  br label %bb33

bb34:                                             ; preds = %bb4
  %37 = load ptr, ptr @0, align 8
  %38 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %37, ptr %_37, align 8
  %39 = getelementptr inbounds i8, ptr %_37, i64 8
  store i64 %38, ptr %39, align 8
  br label %bb33

bb33:                                             ; preds = %bb34, %bb35
  %40 = load ptr, ptr %_37, align 8
  %41 = ptrtoint ptr %40 to i64
  %42 = icmp eq i64 %41, 0
  %_39 = select i1 %42, i64 1, i64 0
  %43 = icmp eq i64 %_39, 0
  br i1 %43, label %bb9, label %bb10

bb9:                                              ; preds = %bb33
  %new_ptr.0 = load ptr, ptr %_37, align 8
  %44 = getelementptr inbounds i8, ptr %_37, i64 8
  %new_ptr.1 = load i64, ptr %44, align 8
  br label %bb36

bb10:                                             ; preds = %bb33
  %45 = load ptr, ptr @0, align 8
  %46 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %45, ptr %_0, align 8
  %47 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %46, ptr %47, align 8
  br label %bb11

bb36:                                             ; preds = %bb9
  %48 = load i64, ptr %old_size, align 8
; call core::intrinsics::copy_nonoverlapping::precondition_check
  call void @_ZN4core10intrinsics19copy_nonoverlapping18precondition_check17hbcc74fdbdf0ffda5E(ptr %ptr, ptr %new_ptr.0, i64 1, i64 1, i64 %48) #17
  br label %bb38

bb38:                                             ; preds = %bb36
  %49 = load i64, ptr %old_size, align 8
  %50 = mul i64 %49, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %new_ptr.0, ptr align 1 %ptr, i64 %50, i1 false)
  %51 = load i64, ptr %old_layout, align 8
  %52 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %53 = load i64, ptr %52, align 8
  store i64 %51, ptr %layout7, align 8
  %54 = getelementptr inbounds i8, ptr %layout7, i64 8
  store i64 %53, ptr %54, align 8
  %55 = load i64, ptr %old_size, align 8
  %56 = icmp eq i64 %55, 0
  br i1 %56, label %bb40, label %bb39

bb40:                                             ; preds = %bb39, %bb38
  store ptr %new_ptr.0, ptr %_0, align 8
  %57 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %new_ptr.1, ptr %57, align 8
  br label %bb11

bb39:                                             ; preds = %bb38
  %58 = load i64, ptr %old_layout, align 8
  %59 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %60 = load i64, ptr %59, align 8
  store i64 %58, ptr %layout14, align 8
  %61 = getelementptr inbounds i8, ptr %layout14, i64 8
  store i64 %60, ptr %61, align 8
  %62 = load i64, ptr %old_size, align 8
  call void @__rust_dealloc(ptr %ptr, i64 %62, i64 %_47) #17
  br label %bb40

bb12:                                             ; preds = %bb3
; call core::hint::assert_unchecked::precondition_check
  call void @_ZN4core4hint16assert_unchecked18precondition_check17h286af9399513f8f3E(i1 zeroext %cond) #17
  store ptr %ptr, ptr %self2, align 8
  store ptr %ptr, ptr %_59, align 8
  %63 = load ptr, ptr %_59, align 8
  store ptr %63, ptr %ptr1, align 8
  %64 = load i64, ptr %old_layout, align 8
  %65 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %66 = load i64, ptr %65, align 8
  store i64 %64, ptr %layout, align 8
  %67 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %66, ptr %67, align 8
  store i64 %new_size17, ptr %new_size, align 8
  store ptr %layout, ptr %self8, align 8
  store ptr %layout, ptr %self9, align 8
  store i64 %self15, ptr %self10, align 8
  %68 = load ptr, ptr %ptr1, align 8
  %69 = load i64, ptr %old_size, align 8
  %70 = call ptr @__rust_realloc(ptr %68, i64 %69, i64 %_47, i64 %new_size17) #17
  store ptr %70, ptr %raw_ptr, align 8
  %71 = load ptr, ptr %raw_ptr, align 8
  store ptr %71, ptr %ptr5, align 8
  %72 = load ptr, ptr %raw_ptr, align 8
  store ptr %72, ptr %ptr11, align 8
  %73 = load ptr, ptr %raw_ptr, align 8
  %74 = ptrtoint ptr %73 to i64
  store i64 %74, ptr %_65, align 8
  %75 = load i64, ptr %_65, align 8
  %76 = icmp eq i64 %75, 0
  br i1 %76, label %bb16, label %bb45

bb13:                                             ; No predecessors!
  unreachable

bb14:                                             ; No predecessors!
  unreachable

bb17:                                             ; No predecessors!
  unreachable

bb16:                                             ; preds = %bb12
  store ptr null, ptr %self4, align 8
  br label %bb15

bb45:                                             ; preds = %bb12
  br label %bb18

bb15:                                             ; preds = %bb20, %bb16
  %77 = load ptr, ptr %self4, align 8
  %78 = ptrtoint ptr %77 to i64
  %79 = icmp eq i64 %78, 0
  %_69 = select i1 %79, i64 0, i64 1
  %80 = icmp eq i64 %_69, 0
  br i1 %80, label %bb22, label %bb23

bb18:                                             ; preds = %bb45
  %_67 = load ptr, ptr %raw_ptr, align 8
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %_67) #17
  br label %bb20

bb20:                                             ; preds = %bb18
  %_68 = load ptr, ptr %raw_ptr, align 8
  store ptr %_68, ptr %self4, align 8
  br label %bb15

bb22:                                             ; preds = %bb15
  store ptr null, ptr %self3, align 8
  br label %bb21

bb23:                                             ; preds = %bb15
  %v = load ptr, ptr %self4, align 8
  store ptr %v, ptr %self3, align 8
  br label %bb21

bb21:                                             ; preds = %bb23, %bb22
  %81 = load ptr, ptr %self3, align 8
  %82 = ptrtoint ptr %81 to i64
  %83 = icmp eq i64 %82, 0
  %_71 = select i1 %83, i64 1, i64 0
  %84 = icmp eq i64 %_71, 0
  br i1 %84, label %bb25, label %bb24

bb25:                                             ; preds = %bb21
  %v18 = load ptr, ptr %self3, align 8
  store ptr %v18, ptr %_25, align 8
  %ptr19 = load ptr, ptr %_25, align 8
  br i1 %zeroed, label %bb6, label %bb7

bb24:                                             ; preds = %bb21
  %85 = load ptr, ptr @0, align 8
  %86 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %85, ptr %_0, align 8
  %87 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %86, ptr %87, align 8
  br label %bb11

bb7:                                              ; preds = %bb29, %bb25
  store ptr %ptr19, ptr %data, align 8
  store i64 %new_size17, ptr %len, align 8
  store ptr %ptr19, ptr %_77, align 8
  %88 = load ptr, ptr %_77, align 8
  store ptr %88, ptr %data13, align 8
  %89 = load ptr, ptr %data13, align 8
  store ptr %89, ptr %ptr12, align 8
  %90 = getelementptr inbounds i8, ptr %ptr12, i64 8
  store i64 %new_size17, ptr %90, align 8
  br label %bb30

bb6:                                              ; preds = %bb25
  %self20 = load ptr, ptr %raw_ptr, align 8
  %91 = load ptr, ptr %raw_ptr, align 8
  %92 = load i64, ptr %old_size, align 8
  %self21 = getelementptr inbounds i8, ptr %91, i64 %92
  %93 = load i64, ptr %old_size, align 8
  %count = sub i64 %new_size17, %93
  br label %bb27

bb27:                                             ; preds = %bb6
; call core::intrinsics::write_bytes::precondition_check
  call void @_ZN4core10intrinsics11write_bytes18precondition_check17h0552f236356537ffE(ptr %self21, i64 1) #17
  %94 = mul i64 1, %count
  call void @llvm.memset.p0.i64(ptr align 1 %self21, i8 0, i64 %94, i1 false)
  store ptr %ptr19, ptr %data, align 8
  store i64 %new_size17, ptr %len, align 8
  store ptr %ptr19, ptr %_77, align 8
  %95 = load ptr, ptr %_77, align 8
  store ptr %95, ptr %data13, align 8
  %96 = load ptr, ptr %data13, align 8
  store ptr %96, ptr %ptr12, align 8
  %97 = getelementptr inbounds i8, ptr %ptr12, i64 8
  store i64 %new_size17, ptr %97, align 8
  br label %bb30

bb29:                                             ; No predecessors!
  %98 = mul i64 1, %count
  call void @llvm.memset.p0.i64(ptr align 1 %self21, i8 0, i64 %98, i1 false)
  br label %bb7

bb30:                                             ; preds = %bb27, %bb7
  %_79 = load ptr, ptr %_77, align 8
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %_79) #17
  br label %bb32

bb32:                                             ; preds = %bb30
  %_80.0 = load ptr, ptr %ptr12, align 8
  %99 = getelementptr inbounds i8, ptr %ptr12, i64 8
  %_80.1 = load i64, ptr %99, align 8
  store ptr %_80.0, ptr %_0, align 8
  %100 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %_80.1, ptr %100, align 8
  br label %bb11

bb5:                                              ; No predecessors!
  unreachable
}

; alloc::slice::<impl [T]>::into_vec
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17hd6b9e4db74e020bfE"(ptr sret([24 x i8]) align 8 %_0, ptr align 4 %self.0, i64 %self.1) unnamed_addr #1 {
start:
; call alloc::slice::hack::into_vec
  call void @_ZN5alloc5slice4hack8into_vec17h256a9b845a7e2111E(ptr sret([24 x i8]) align 8 %_0, ptr align 4 %self.0, i64 %self.1)
  ret void
}

; alloc::slice::hack::into_vec
; Function Attrs: nonlazybind uwtable
define internal void @_ZN5alloc5slice4hack8into_vec17h256a9b845a7e2111E(ptr sret([24 x i8]) align 8 %_0, ptr align 4 %b.0, i64 %b.1) unnamed_addr #0 {
start:
  %capacity = alloca [8 x i8], align 8
  %b = alloca [16 x i8], align 8
  store ptr %b.0, ptr %b, align 8
  %0 = getelementptr inbounds i8, ptr %b, i64 8
  store i64 %b.1, ptr %0, align 8
  %_23.0 = load ptr, ptr %b, align 8
  %1 = getelementptr inbounds i8, ptr %b, i64 8
  %_23.1 = load i64, ptr %1, align 8
  %src = getelementptr inbounds i8, ptr %b, i64 16
  br label %bb3

bb3:                                              ; preds = %start
  store i64 %b.1, ptr %capacity, align 8
  br label %bb1

bb1:                                              ; preds = %bb3
  %cap = load i64, ptr %capacity, align 8
  br label %bb4

bb2:                                              ; No predecessors!
  unreachable

bb4:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17hc43869dfcbb4d22cE"(ptr %_23.0) #17
  br label %bb6

bb6:                                              ; preds = %bb4
  store i64 %cap, ptr %_0, align 8
  %2 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr %_23.0, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 16
  store i64 %b.1, ptr %3, align 8
  ret void
}

; alloc::raw_vec::finish_grow
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN5alloc7raw_vec11finish_grow17ha8417367ae761f37E(ptr sret([24 x i8]) align 8 %_0, i64 %0, i64 %1, ptr align 8 %current_memory, ptr align 1 %alloc) unnamed_addr #2 {
start:
  %self1 = alloca [16 x i8], align 8
  %_35 = alloca [8 x i8], align 8
  %_29 = alloca [8 x i8], align 8
  %self = alloca [16 x i8], align 8
  %old_layout = alloca [16 x i8], align 8
  %memory = alloca [16 x i8], align 8
  %new_layout = alloca [16 x i8], align 8
  store i64 %0, ptr %new_layout, align 8
  %2 = getelementptr inbounds i8, ptr %new_layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %alloc_size = load i64, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %current_memory, i64 8
  %5 = load i64, ptr %4, align 8
  %6 = icmp eq i64 %5, 0
  %_9 = select i1 %6, i64 0, i64 1
  %7 = icmp eq i64 %_9, 1
  br i1 %7, label %bb3, label %bb2

bb3:                                              ; preds = %start
  %ptr = load ptr, ptr %current_memory, align 8
  %8 = getelementptr inbounds i8, ptr %current_memory, i64 8
  %9 = load i64, ptr %8, align 8
  %10 = getelementptr inbounds i8, ptr %8, i64 8
  %11 = load i64, ptr %10, align 8
  store i64 %9, ptr %old_layout, align 8
  %12 = getelementptr inbounds i8, ptr %old_layout, i64 8
  store i64 %11, ptr %12, align 8
  %self2 = load i64, ptr %old_layout, align 8
  store i64 %self2, ptr %_29, align 8
  %_30 = load i64, ptr %_29, align 8
  %_31 = icmp uge i64 %_30, 1
  %_32 = icmp ule i64 %_30, -9223372036854775808
  %_33 = and i1 %_31, %_32
  %self3 = load i64, ptr %new_layout, align 8
  store i64 %self3, ptr %_35, align 8
  %_36 = load i64, ptr %_35, align 8
  %_37 = icmp uge i64 %_36, 1
  %_38 = icmp ule i64 %_36, -9223372036854775808
  %_39 = and i1 %_37, %_38
  %cond = icmp eq i64 %_30, %_36
  br label %bb7

bb2:                                              ; preds = %start
  %13 = load i64, ptr %new_layout, align 8
  %14 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %15 = load i64, ptr %14, align 8
; call <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %16 = call { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h95fc45768c95ad92E"(ptr align 1 %alloc, i64 %13, i64 %15)
  %17 = extractvalue { ptr, i64 } %16, 0
  %18 = extractvalue { ptr, i64 } %16, 1
  store ptr %17, ptr %memory, align 8
  %19 = getelementptr inbounds i8, ptr %memory, i64 8
  store i64 %18, ptr %19, align 8
  br label %bb6

bb7:                                              ; preds = %bb3
; call core::hint::assert_unchecked::precondition_check
  call void @_ZN4core4hint16assert_unchecked18precondition_check17h286af9399513f8f3E(i1 zeroext %cond) #17
  br label %bb8

bb8:                                              ; preds = %bb7
  %20 = load i64, ptr %old_layout, align 8
  %21 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %22 = load i64, ptr %21, align 8
  %23 = load i64, ptr %new_layout, align 8
  %24 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %25 = load i64, ptr %24, align 8
; call <alloc::alloc::Global as core::alloc::Allocator>::grow
  %26 = call { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h9a01680d21bf2ef1E"(ptr align 1 %alloc, ptr %ptr, i64 %20, i64 %22, i64 %23, i64 %25)
  %27 = extractvalue { ptr, i64 } %26, 0
  %28 = extractvalue { ptr, i64 } %26, 1
  store ptr %27, ptr %memory, align 8
  %29 = getelementptr inbounds i8, ptr %memory, i64 8
  store i64 %28, ptr %29, align 8
  br label %bb6

bb6:                                              ; preds = %bb2, %bb8
  %30 = load ptr, ptr %memory, align 8
  %31 = getelementptr inbounds i8, ptr %memory, i64 8
  %32 = load i64, ptr %31, align 8
  store ptr %30, ptr %self, align 8
  %33 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %32, ptr %33, align 8
  %34 = load ptr, ptr %self, align 8
  %35 = ptrtoint ptr %34 to i64
  %36 = icmp eq i64 %35, 0
  %_42 = select i1 %36, i64 1, i64 0
  %37 = icmp eq i64 %_42, 0
  br i1 %37, label %bb11, label %bb10

bb11:                                             ; preds = %bb6
  %t.0 = load ptr, ptr %self, align 8
  %38 = getelementptr inbounds i8, ptr %self, i64 8
  %t.1 = load i64, ptr %38, align 8
  %39 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr %t.0, ptr %39, align 8
  %40 = getelementptr inbounds i8, ptr %39, i64 8
  store i64 %t.1, ptr %40, align 8
  store i64 0, ptr %_0, align 8
  br label %bb9

bb10:                                             ; preds = %bb6
  %41 = load i64, ptr %new_layout, align 8
  %42 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %43 = load i64, ptr %42, align 8
  store i64 %41, ptr %self1, align 8
  %44 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %43, ptr %44, align 8
  %_44.0 = load i64, ptr %self1, align 8
  %45 = getelementptr inbounds i8, ptr %self1, i64 8
  %_44.1 = load i64, ptr %45, align 8
  %46 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %_44.0, ptr %46, align 8
  %47 = getelementptr inbounds i8, ptr %46, i64 8
  store i64 %_44.1, ptr %47, align 8
  store i64 1, ptr %_0, align 8
  br label %bb9

bb9:                                              ; preds = %bb10, %bb11
  ret void

bb1:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::grow_one
; Function Attrs: noinline nonlazybind uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h75d06ceed2b17565E"(ptr align 8 %self) unnamed_addr #2 {
start:
  %_3 = alloca [16 x i8], align 8
  %_4 = load i64, ptr %self, align 8
; call alloc::raw_vec::RawVecInner<A>::grow_amortized
  %0 = call { i64, i64 } @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17h45485fd1ac00a956E"(ptr align 8 %self, i64 %_4, i64 1, i64 8, i64 8)
  %1 = extractvalue { i64, i64 } %0, 0
  %2 = extractvalue { i64, i64 } %0, 1
  store i64 %1, ptr %_3, align 8
  %3 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 %2, ptr %3, align 8
  %4 = load i64, ptr %_3, align 8
  %5 = icmp eq i64 %4, -9223372036854775807
  %_5 = select i1 %5, i64 0, i64 1
  %6 = icmp eq i64 %_5, 1
  br i1 %6, label %bb2, label %bb3

bb2:                                              ; preds = %start
  %err.0 = load i64, ptr %_3, align 8
  %7 = getelementptr inbounds i8, ptr %_3, i64 8
  %err.1 = load i64, ptr %7, align 8
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17he4316ba2e8167751E(i64 %err.0, i64 %err.1) #19
  unreachable

bb3:                                              ; preds = %start
  ret void

bb4:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::grow_one
; Function Attrs: noinline nonlazybind uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h7f186cc5654d5cf9E"(ptr align 8 %self) unnamed_addr #2 {
start:
  %_3 = alloca [16 x i8], align 8
  %_4 = load i64, ptr %self, align 8
; call alloc::raw_vec::RawVecInner<A>::grow_amortized
  %0 = call { i64, i64 } @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17h45485fd1ac00a956E"(ptr align 8 %self, i64 %_4, i64 1, i64 4, i64 4)
  %1 = extractvalue { i64, i64 } %0, 0
  %2 = extractvalue { i64, i64 } %0, 1
  store i64 %1, ptr %_3, align 8
  %3 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 %2, ptr %3, align 8
  %4 = load i64, ptr %_3, align 8
  %5 = icmp eq i64 %4, -9223372036854775807
  %_5 = select i1 %5, i64 0, i64 1
  %6 = icmp eq i64 %_5, 1
  br i1 %6, label %bb2, label %bb3

bb2:                                              ; preds = %start
  %err.0 = load i64, ptr %_3, align 8
  %7 = getelementptr inbounds i8, ptr %_3, i64 8
  %err.1 = load i64, ptr %7, align 8
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17he4316ba2e8167751E(i64 %err.0, i64 %err.1) #19
  unreachable

bb3:                                              ; preds = %start
  ret void

bb4:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVecInner<A>::deallocate
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$10deallocate17h3616287c4325bdeaE"(ptr align 8 %self, i64 %elem_layout.0, i64 %elem_layout.1) unnamed_addr #0 {
start:
  %_3 = alloca [24 x i8], align 8
; call alloc::raw_vec::RawVecInner<A>::current_memory
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hb542f1f330ae991cE"(ptr sret([24 x i8]) align 8 %_3, ptr align 8 %self, i64 %elem_layout.0, i64 %elem_layout.1)
  %0 = getelementptr inbounds i8, ptr %_3, i64 8
  %1 = load i64, ptr %0, align 8
  %2 = icmp eq i64 %1, 0
  %_5 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_5, 1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %start
  %ptr = load ptr, ptr %_3, align 8
  %4 = getelementptr inbounds i8, ptr %_3, i64 8
  %layout.0 = load i64, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %4, i64 8
  %layout.1 = load i64, ptr %5, align 8
  %_9 = getelementptr inbounds i8, ptr %self, i64 16
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h3f158fa8a2455d65E"(ptr align 1 %_9, ptr %ptr, i64 %layout.0, i64 %layout.1)
  br label %bb4

bb4:                                              ; preds = %bb2, %start
  ret void

bb5:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVecInner<A>::current_memory
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hb542f1f330ae991cE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %self, i64 %0, i64 %1) unnamed_addr #1 {
start:
  %_21 = alloca [1 x i8], align 1
  %_20 = alloca [1 x i8], align 1
  %_19 = alloca [1 x i8], align 1
  %_18 = alloca [8 x i8], align 8
  %_17 = alloca [8 x i8], align 8
  %self2 = alloca [8 x i8], align 8
  %_13 = alloca [24 x i8], align 8
  %self1 = alloca [8 x i8], align 8
  %align = alloca [8 x i8], align 8
  %size = alloca [8 x i8], align 8
  %alloc_size = alloca [8 x i8], align 8
  %elem_layout = alloca [16 x i8], align 8
  store i64 %0, ptr %elem_layout, align 8
  %2 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  %self3 = load i64, ptr %3, align 8
  %4 = icmp eq i64 %self3, 0
  br i1 %4, label %bb3, label %bb1

bb3:                                              ; preds = %bb2, %start
  %5 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 0, ptr %5, align 8
  br label %bb5

bb1:                                              ; preds = %start
  %_5 = load i64, ptr %self, align 8
  %6 = icmp eq i64 %_5, 0
  br i1 %6, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  br label %bb3

bb4:                                              ; preds = %bb1
  %rhs = load i64, ptr %self, align 8
  br label %bb6

bb5:                                              ; preds = %bb9, %bb3
  ret void

bb6:                                              ; preds = %bb4
; call core::num::<impl usize>::unchecked_mul::precondition_check
  call void @"_ZN4core3num23_$LT$impl$u20$usize$GT$13unchecked_mul18precondition_check17hefe36670947257c6E"(i64 %self3, i64 %rhs) #17
  %7 = mul nuw i64 %self3, %rhs
  store i64 %7, ptr %alloc_size, align 8
  %8 = load i64, ptr %alloc_size, align 8
  store i64 %8, ptr %size, align 8
  store ptr %elem_layout, ptr %self1, align 8
  %9 = load i64, ptr %elem_layout, align 8
  store i64 %9, ptr %self2, align 8
  %10 = load i64, ptr %self2, align 8
  store i64 %10, ptr %_17, align 8
  %11 = load i64, ptr %_17, align 8
  store i64 %11, ptr %_18, align 8
  %12 = load i64, ptr %_18, align 8
  %13 = icmp uge i64 %12, 1
  %14 = zext i1 %13 to i8
  store i8 %14, ptr %_19, align 1
  %15 = load i64, ptr %_18, align 8
  %16 = icmp ule i64 %15, -9223372036854775808
  %17 = zext i1 %16 to i8
  store i8 %17, ptr %_20, align 1
  %18 = load i8, ptr %_19, align 1
  %19 = trunc i8 %18 to i1
  %20 = load i8, ptr %_20, align 1
  %21 = trunc i8 %20 to i1
  %22 = and i1 %19, %21
  %23 = zext i1 %22 to i8
  store i8 %23, ptr %_21, align 1
  %24 = load i64, ptr %_18, align 8
  store i64 %24, ptr %align, align 8
  br label %bb8

bb7:                                              ; No predecessors!
  unreachable

bb8:                                              ; preds = %bb6
  %25 = load i64, ptr %alloc_size, align 8
  %26 = load i64, ptr %align, align 8
; call core::alloc::layout::Layout::from_size_align_unchecked::precondition_check
  call void @_ZN4core5alloc6layout6Layout25from_size_align_unchecked18precondition_check17hfd805233854ca994E(i64 %25, i64 %26) #17
  br label %bb9

bb9:                                              ; preds = %bb8
  %_23 = load i64, ptr %align, align 8
  %layout.1 = load i64, ptr %alloc_size, align 8
  %27 = getelementptr inbounds i8, ptr %self, i64 8
  %self4 = load ptr, ptr %27, align 8
  store ptr %self4, ptr %_13, align 8
  %28 = getelementptr inbounds i8, ptr %_13, i64 8
  store i64 %_23, ptr %28, align 8
  %29 = getelementptr inbounds i8, ptr %28, i64 8
  store i64 %layout.1, ptr %29, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_13, i64 24, i1 false)
  br label %bb5
}

; alloc::raw_vec::RawVecInner<A>::grow_amortized
; Function Attrs: nonlazybind uwtable
define internal { i64, i64 } @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14grow_amortized17h45485fd1ac00a956E"(ptr align 8 %self, i64 %len, i64 %additional, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %2 = alloca [1 x i8], align 1
  %_55 = alloca [16 x i8], align 8
  %_50 = alloca [16 x i8], align 8
  %self9 = alloca [24 x i8], align 8
  %self8 = alloca [16 x i8], align 8
  %_37 = alloca [16 x i8], align 8
  %residual7 = alloca [16 x i8], align 8
  %_25 = alloca [24 x i8], align 8
  %self6 = alloca [24 x i8], align 8
  %_23 = alloca [24 x i8], align 8
  %residual5 = alloca [16 x i8], align 8
  %elem_layout4 = alloca [16 x i8], align 8
  %self3 = alloca [24 x i8], align 8
  %_18 = alloca [24 x i8], align 8
  %v1 = alloca [8 x i8], align 8
  %residual = alloca [16 x i8], align 8
  %self2 = alloca [16 x i8], align 8
  %self1 = alloca [16 x i8], align 8
  %_7 = alloca [16 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %elem_layout = alloca [16 x i8], align 8
  store i64 %0, ptr %elem_layout, align 8
  %3 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  store i64 %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  %size = load i64, ptr %4, align 8
  %5 = icmp eq i64 %size, 0
  br i1 %5, label %bb1, label %bb2

bb1:                                              ; preds = %start
  %6 = load i64, ptr @0, align 8
  %7 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %6, ptr %_0, align 8
  %8 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %7, ptr %8, align 8
  br label %bb8

bb2:                                              ; preds = %start
  %9 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %len, i64 %additional)
  %_32.0 = extractvalue { i64, i1 } %9, 0
  %_32.1 = extractvalue { i64, i1 } %9, 1
  %10 = call i1 @llvm.expect.i1(i1 %_32.1, i1 false)
  %11 = zext i1 %10 to i8
  store i8 %11, ptr %2, align 1
  %12 = load i8, ptr %2, align 1
  %_30 = trunc i8 %12 to i1
  br i1 %_30, label %bb10, label %bb11

bb8:                                              ; preds = %bb7, %bb24, %bb1
  %13 = load i64, ptr %_0, align 8
  %14 = getelementptr inbounds i8, ptr %_0, i64 8
  %15 = load i64, ptr %14, align 8
  %16 = insertvalue { i64, i64 } poison, i64 %13, 0
  %17 = insertvalue { i64, i64 } %16, i64 %15, 1
  ret { i64, i64 } %17

bb11:                                             ; preds = %bb2
  %_33 = add nuw i64 %len, %additional
  %18 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %_33, ptr %18, align 8
  store i64 1, ptr %self2, align 8
  %19 = getelementptr inbounds i8, ptr %self2, i64 8
  %v = load i64, ptr %19, align 8
  %20 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %v, ptr %20, align 8
  store i64 -9223372036854775807, ptr %self1, align 8
  %21 = getelementptr inbounds i8, ptr %self1, i64 8
  %v10 = load i64, ptr %21, align 8
  %22 = getelementptr inbounds i8, ptr %_7, i64 8
  store i64 %v10, ptr %22, align 8
  store i64 -9223372036854775807, ptr %_7, align 8
  %23 = getelementptr inbounds i8, ptr %_7, i64 8
  %required_cap = load i64, ptr %23, align 8
  %_14 = load i64, ptr %self, align 8
  %v111 = mul i64 %_14, 2
; call core::cmp::max_by
  %cap = call i64 @_ZN4core3cmp6max_by17hadf1ac163bd5bd80E(i64 %v111, i64 %required_cap)
  %24 = icmp eq i64 %size, 1
  br i1 %24, label %bb14, label %bb15

bb10:                                             ; preds = %bb2
  %25 = load i64, ptr @0, align 8
  %26 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %25, ptr %self2, align 8
  %27 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %26, ptr %27, align 8
  %28 = load i64, ptr @0, align 8
  %29 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %28, ptr %self1, align 8
  %30 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %29, ptr %30, align 8
  %e.023 = load i64, ptr %self1, align 8
  %31 = getelementptr inbounds i8, ptr %self1, i64 8
  %e.124 = load i64, ptr %31, align 8
  store i64 %e.023, ptr %_37, align 8
  %32 = getelementptr inbounds i8, ptr %_37, i64 8
  store i64 %e.124, ptr %32, align 8
  %33 = load i64, ptr %_37, align 8
  %34 = getelementptr inbounds i8, ptr %_37, i64 8
  %35 = load i64, ptr %34, align 8
  store i64 %33, ptr %_7, align 8
  %36 = getelementptr inbounds i8, ptr %_7, i64 8
  store i64 %35, ptr %36, align 8
  %37 = load i64, ptr %_7, align 8
  %38 = getelementptr inbounds i8, ptr %_7, i64 8
  %39 = load i64, ptr %38, align 8
  store i64 %37, ptr %residual, align 8
  %40 = getelementptr inbounds i8, ptr %residual, i64 8
  store i64 %39, ptr %40, align 8
  %e.025 = load i64, ptr %residual, align 8
  %41 = getelementptr inbounds i8, ptr %residual, i64 8
  %e.126 = load i64, ptr %41, align 8
  store i64 %e.025, ptr %_0, align 8
  %42 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %e.126, ptr %42, align 8
  br label %bb7

bb14:                                             ; preds = %bb11
  store i64 8, ptr %v1, align 8
  br label %bb13

bb15:                                             ; preds = %bb11
  %_40 = icmp ule i64 %size, 1024
  br i1 %_40, label %bb16, label %bb17

bb13:                                             ; preds = %bb18, %bb14
  %43 = load i64, ptr %v1, align 8
; call core::cmp::max_by
  %cap12 = call i64 @_ZN4core3cmp6max_by17hadf1ac163bd5bd80E(i64 %43, i64 %cap)
  %44 = load i64, ptr %elem_layout, align 8
  %45 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  %46 = load i64, ptr %45, align 8
  store i64 %44, ptr %elem_layout4, align 8
  %47 = getelementptr inbounds i8, ptr %elem_layout4, i64 8
  store i64 %46, ptr %47, align 8
; call core::alloc::layout::Layout::repeat
  call void @_ZN4core5alloc6layout6Layout6repeat17h26c4997c792ae0f3E(ptr sret([24 x i8]) align 8 %self9, ptr align 8 %elem_layout4, i64 %cap12)
  %48 = load i64, ptr %self9, align 8
  %49 = icmp eq i64 %48, 0
  %_44 = select i1 %49, i64 1, i64 0
  %50 = icmp eq i64 %_44, 0
  br i1 %50, label %bb22, label %bb21

bb17:                                             ; preds = %bb15
  store i64 1, ptr %v1, align 8
  br label %bb18

bb16:                                             ; preds = %bb15
  store i64 4, ptr %v1, align 8
  br label %bb18

bb18:                                             ; preds = %bb16, %bb17
  br label %bb13

bb22:                                             ; preds = %bb13
  %t.0 = load i64, ptr %self9, align 8
  %51 = getelementptr inbounds i8, ptr %self9, i64 8
  %t.1 = load i64, ptr %51, align 8
  %52 = getelementptr inbounds i8, ptr %self9, i64 16
  %t = load i64, ptr %52, align 8
  store i64 %t.0, ptr %self8, align 8
  %53 = getelementptr inbounds i8, ptr %self8, i64 8
  store i64 %t.1, ptr %53, align 8
  %t.013 = load i64, ptr %self8, align 8
  %54 = getelementptr inbounds i8, ptr %self8, i64 8
  %t.114 = load i64, ptr %54, align 8
  %55 = getelementptr inbounds i8, ptr %self3, i64 8
  store i64 %t.013, ptr %55, align 8
  %56 = getelementptr inbounds i8, ptr %55, i64 8
  store i64 %t.114, ptr %56, align 8
  store i64 0, ptr %self3, align 8
  %57 = getelementptr inbounds i8, ptr %self3, i64 8
  %v.0 = load i64, ptr %57, align 8
  %58 = getelementptr inbounds i8, ptr %57, i64 8
  %v.1 = load i64, ptr %58, align 8
  %59 = getelementptr inbounds i8, ptr %_18, i64 8
  store i64 %v.0, ptr %59, align 8
  %60 = getelementptr inbounds i8, ptr %59, i64 8
  store i64 %v.1, ptr %60, align 8
  store i64 0, ptr %_18, align 8
  %61 = getelementptr inbounds i8, ptr %_18, i64 8
  %new_layout.0 = load i64, ptr %61, align 8
  %62 = getelementptr inbounds i8, ptr %61, i64 8
  %new_layout.1 = load i64, ptr %62, align 8
  %63 = load i64, ptr %elem_layout, align 8
  %64 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  %65 = load i64, ptr %64, align 8
; call alloc::raw_vec::RawVecInner<A>::current_memory
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$14current_memory17hb542f1f330ae991cE"(ptr sret([24 x i8]) align 8 %_25, ptr align 8 %self, i64 %63, i64 %65)
  %_27 = getelementptr inbounds i8, ptr %self, i64 16
; call alloc::raw_vec::finish_grow
  call void @_ZN5alloc7raw_vec11finish_grow17ha8417367ae761f37E(ptr sret([24 x i8]) align 8 %self6, i64 %new_layout.0, i64 %new_layout.1, ptr align 8 %_25, ptr align 1 %_27)
  %_52 = load i64, ptr %self6, align 8
  %66 = icmp eq i64 %_52, 0
  br i1 %66, label %bb24, label %bb23

bb21:                                             ; preds = %bb13
  %67 = load i64, ptr @0, align 8
  %68 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %67, ptr %self8, align 8
  %69 = getelementptr inbounds i8, ptr %self8, i64 8
  store i64 %68, ptr %69, align 8
  %70 = load i64, ptr @0, align 8
  %71 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %72 = getelementptr inbounds i8, ptr %self3, i64 8
  store i64 %70, ptr %72, align 8
  %73 = getelementptr inbounds i8, ptr %72, i64 8
  store i64 %71, ptr %73, align 8
  store i64 1, ptr %self3, align 8
  %74 = getelementptr inbounds i8, ptr %self3, i64 8
  %e.019 = load i64, ptr %74, align 8
  %75 = getelementptr inbounds i8, ptr %74, i64 8
  %e.120 = load i64, ptr %75, align 8
  store i64 %e.019, ptr %_50, align 8
  %76 = getelementptr inbounds i8, ptr %_50, i64 8
  store i64 %e.120, ptr %76, align 8
  %77 = load i64, ptr %_50, align 8
  %78 = getelementptr inbounds i8, ptr %_50, i64 8
  %79 = load i64, ptr %78, align 8
  %80 = getelementptr inbounds i8, ptr %_18, i64 8
  store i64 %77, ptr %80, align 8
  %81 = getelementptr inbounds i8, ptr %80, i64 8
  store i64 %79, ptr %81, align 8
  store i64 1, ptr %_18, align 8
  %82 = getelementptr inbounds i8, ptr %_18, i64 8
  %83 = load i64, ptr %82, align 8
  %84 = getelementptr inbounds i8, ptr %82, i64 8
  %85 = load i64, ptr %84, align 8
  store i64 %83, ptr %residual5, align 8
  %86 = getelementptr inbounds i8, ptr %residual5, i64 8
  store i64 %85, ptr %86, align 8
  %e.021 = load i64, ptr %residual5, align 8
  %87 = getelementptr inbounds i8, ptr %residual5, i64 8
  %e.122 = load i64, ptr %87, align 8
  store i64 %e.021, ptr %_0, align 8
  %88 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %e.122, ptr %88, align 8
  br label %bb6

bb24:                                             ; preds = %bb22
  %89 = getelementptr inbounds i8, ptr %self6, i64 8
  %v.015 = load ptr, ptr %89, align 8
  %90 = getelementptr inbounds i8, ptr %89, i64 8
  %v.116 = load i64, ptr %90, align 8
  %91 = getelementptr inbounds i8, ptr %_23, i64 8
  store ptr %v.015, ptr %91, align 8
  %92 = getelementptr inbounds i8, ptr %91, i64 8
  store i64 %v.116, ptr %92, align 8
  store i64 0, ptr %_23, align 8
  %93 = getelementptr inbounds i8, ptr %_23, i64 8
  %ptr.0 = load ptr, ptr %93, align 8
  %94 = getelementptr inbounds i8, ptr %93, i64 8
  %ptr.1 = load i64, ptr %94, align 8
  %95 = getelementptr inbounds i8, ptr %self, i64 8
  store ptr %ptr.0, ptr %95, align 8
  store i64 %cap12, ptr %self, align 8
  %96 = load i64, ptr @2, align 8
  %97 = load i64, ptr getelementptr inbounds (i8, ptr @2, i64 8), align 8
  store i64 %96, ptr %_0, align 8
  %98 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %97, ptr %98, align 8
  br label %bb8

bb23:                                             ; preds = %bb22
  %99 = getelementptr inbounds i8, ptr %self6, i64 8
  %e.0 = load i64, ptr %99, align 8
  %100 = getelementptr inbounds i8, ptr %99, i64 8
  %e.1 = load i64, ptr %100, align 8
  store i64 %e.0, ptr %_55, align 8
  %101 = getelementptr inbounds i8, ptr %_55, i64 8
  store i64 %e.1, ptr %101, align 8
  %102 = load i64, ptr %_55, align 8
  %103 = getelementptr inbounds i8, ptr %_55, i64 8
  %104 = load i64, ptr %103, align 8
  %105 = getelementptr inbounds i8, ptr %_23, i64 8
  store i64 %102, ptr %105, align 8
  %106 = getelementptr inbounds i8, ptr %105, i64 8
  store i64 %104, ptr %106, align 8
  store i64 1, ptr %_23, align 8
  %107 = getelementptr inbounds i8, ptr %_23, i64 8
  %108 = load i64, ptr %107, align 8
  %109 = getelementptr inbounds i8, ptr %107, i64 8
  %110 = load i64, ptr %109, align 8
  store i64 %108, ptr %residual7, align 8
  %111 = getelementptr inbounds i8, ptr %residual7, i64 8
  store i64 %110, ptr %111, align 8
  %e.017 = load i64, ptr %residual7, align 8
  %112 = getelementptr inbounds i8, ptr %residual7, i64 8
  %e.118 = load i64, ptr %112, align 8
  store i64 %e.017, ptr %_0, align 8
  %113 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %e.118, ptr %113, align 8
  br label %bb6

bb6:                                              ; preds = %bb21, %bb23
  br label %bb7

bb7:                                              ; preds = %bb10, %bb6
  br label %bb8

bb3:                                              ; No predecessors!
  unreachable
}

; <I as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h668e2690d8919983E"(i64 %self.0, i64 %self.1) unnamed_addr #1 {
start:
  %0 = insertvalue { i64, i64 } poison, i64 %self.0, 0
  %1 = insertvalue { i64, i64 } %0, i64 %self.1, 1
  ret { i64, i64 } %1
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h3f158fa8a2455d65E"(ptr align 1 %self, ptr %ptr, i64 %0, i64 %1) unnamed_addr #1 {
start:
  %_13 = alloca [8 x i8], align 8
  %layout1 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %_4 = load i64, ptr %3, align 8
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  %5 = load i64, ptr %layout, align 8
  %6 = getelementptr inbounds i8, ptr %layout, i64 8
  %7 = load i64, ptr %6, align 8
  store i64 %5, ptr %layout1, align 8
  %8 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %7, ptr %8, align 8
  %self2 = load i64, ptr %layout, align 8
  store i64 %self2, ptr %_13, align 8
  %_14 = load i64, ptr %_13, align 8
  %_15 = icmp uge i64 %_14, 1
  %_16 = icmp ule i64 %_14, -9223372036854775808
  %_17 = and i1 %_15, %_16
  call void @__rust_dealloc(ptr %ptr, i64 %_4, i64 %_14) #17
  br label %bb2
}

; <alloc::alloc::Global as core::alloc::Allocator>::grow
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h9a01680d21bf2ef1E"(ptr align 1 %self, ptr %ptr, i64 %old_layout.0, i64 %old_layout.1, i64 %new_layout.0, i64 %new_layout.1) unnamed_addr #1 {
start:
; call alloc::alloc::Global::grow_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global9grow_impl17hc4ab2c2a56faffa1E(ptr align 1 %self, ptr %ptr, i64 %old_layout.0, i64 %old_layout.1, i64 %new_layout.0, i64 %new_layout.1, i1 zeroext false)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h95fc45768c95ad92E"(ptr align 1 %self, i64 %layout.0, i64 %layout.1) unnamed_addr #1 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h6f3bfdc26261c320E(ptr align 1 %self, i64 %layout.0, i64 %layout.1, i1 zeroext false)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h9a592e2466e71906E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %0, align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8
  ret void
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb2bb135c24939ec8E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %0, align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8
  ret void
}

; <usize as core::slice::index::SliceIndex<[T]>>::index
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 8 ptr @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hc3092223740c8d17E"(i64 %self, ptr align 8 %slice.0, i64 %slice.1, ptr align 8 %0) unnamed_addr #1 {
start:
  %_4 = icmp ult i64 %self, %slice.1
  br i1 %_4, label %bb1, label %panic

bb1:                                              ; preds = %start
  %_0 = getelementptr inbounds [0 x i64], ptr %slice.0, i64 0, i64 %self
  ret ptr %_0

panic:                                            ; preds = %start
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17hab02a8df06d3a143E(i64 %self, i64 %slice.1, ptr align 8 %0) #19
  unreachable
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8e3ad2006b99dd9eE"(ptr align 8 %self) unnamed_addr #0 {
start:
; call alloc::raw_vec::RawVecInner<A>::deallocate
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$10deallocate17h3616287c4325bdeaE"(ptr align 8 %self, i64 8, i64 8)
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hfae5577759daab49E"(ptr align 8 %self) unnamed_addr #0 {
start:
; call alloc::raw_vec::RawVecInner<A>::deallocate
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$10deallocate17h3616287c4325bdeaE"(ptr align 8 %self, i64 4, i64 4)
  ret void
}

; <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 8 ptr @"_ZN81_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..index..Index$LT$I$GT$$GT$5index17h54e90fe376be51fbE"(ptr align 8 %self, i64 %index, ptr align 8 %0) unnamed_addr #1 {
start:
  %1 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %2, align 8
  br label %bb1

bb1:                                              ; preds = %start
; call core::slice::raw::from_raw_parts::precondition_check
  call void @_ZN4core5slice3raw14from_raw_parts18precondition_check17h3aa1340e63dfc8edE(ptr %self1, i64 8, i64 8, i64 %len) #17
  br label %bb3

bb3:                                              ; preds = %bb1
; call <usize as core::slice::index::SliceIndex<[T]>>::index
  %_0 = call align 8 ptr @"_ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17hc3092223740c8d17E"(i64 %index, ptr align 8 %self1, i64 %len, ptr align 8 %0)
  ret ptr %_0
}

; <core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @"_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17hc3170b232b7c34a2E"(ptr align 8 %self) unnamed_addr #1 {
start:
  %_0 = alloca [16 x i8], align 8
  %_4 = getelementptr inbounds i8, ptr %self, i64 8
  %_3.i = load i64, ptr %self, align 8
  %_4.i = load i64, ptr %_4, align 8
  %_0.i = icmp ult i64 %_3.i, %_4.i
  br i1 %_0.i, label %bb2, label %bb4

bb4:                                              ; preds = %start
  store i64 0, ptr %_0, align 8
  br label %bb5

bb2:                                              ; preds = %start
  %old = load i64, ptr %self, align 8
; call <usize as core::iter::range::Step>::forward_unchecked
  %_6 = call i64 @"_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17hddb5e65572d7ba20E"(i64 %old, i64 1)
  store i64 %_6, ptr %self, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %old, ptr %0, align 8
  store i64 1, ptr %_0, align 8
  br label %bb5

bb5:                                              ; preds = %bb2, %bb4
  %1 = load i64, ptr %_0, align 8
  %2 = getelementptr inbounds i8, ptr %_0, i64 8
  %3 = load i64, ptr %2, align 8
  %4 = insertvalue { i64, i64 } poison, i64 %1, 0
  %5 = insertvalue { i64, i64 } %4, i64 %3, 1
  ret { i64, i64 } %5
}

; exec_v1::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN7exec_v14main17hd1092b66ca4bfeb0E() unnamed_addr #0 {
start:
; call exec_v1::tests::f
  call void @_ZN7exec_v15tests1f17h95473cd642c557a4E()
  ret void
}

; exec_v1::Borrow<T>::read
; Function Attrs: nonlazybind uwtable
define internal align 4 ptr @"_ZN7exec_v115Borrow$LT$T$GT$4read17h4fb74ce01bd3bda2E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_2 = load i64, ptr %0, align 8
  %ptr.i = inttoptr i64 %_2 to ptr
  ret ptr %ptr.i
}

; exec_v1::Borrow<T>::read::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 1 ptr @"_ZN7exec_v115Borrow$LT$T$GT$4read28_$u7b$$u7b$closure$u7d$$u7d$17h55faaa3ca3f2bb34E"(ptr align 1 %_1) unnamed_addr #1 {
start:
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h102d65dbfa674afeE(ptr align 1 @alloc_a500d906b91607583596fa15e63c2ada, i64 40, ptr align 8 @alloc_466e6bf9c467b28455453eef8ceb5712) #19
  unreachable
}

; exec_v1::Borrow<T>::replace
; Function Attrs: nonlazybind uwtable
define internal i32 @"_ZN7exec_v115Borrow$LT$T$GT$7replace17hdbf402f7ee34963bE"(ptr align 8 %self, i32 %x) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_7 = alloca [1 x i8], align 1
  store i8 0, ptr %_7, align 1
  store i8 1, ptr %_7, align 1
  %1 = getelementptr inbounds i8, ptr %self, i64 8
  %_3 = load i64, ptr %1, align 8
  br label %bb1

bb5:                                              ; preds = %cleanup
  %2 = load i8, ptr %_7, align 1
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb3

cleanup:                                          ; No predecessors!
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = extractvalue { ptr, i32 } %4, 0
  %6 = extractvalue { ptr, i32 } %4, 1
  store ptr %5, ptr %0, align 8
  %7 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %6, ptr %7, align 8
  br label %bb5

bb1:                                              ; preds = %start
  store i8 0, ptr %_7, align 1
  %ptr.i = inttoptr i64 %_3 to ptr
  %out_v.i = load i32, ptr %ptr.i, align 4
  store i32 %x, ptr %ptr.i, align 4
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i32 %out_v.i

bb3:                                              ; preds = %bb4, %bb5
  %8 = load ptr, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  %10 = load i32, ptr %9, align 8
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1
  resume { ptr, i32 } %12

bb4:                                              ; preds = %bb5
  br label %bb3
}

; exec_v1::Borrow<T>::replace::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 1 ptr @"_ZN7exec_v115Borrow$LT$T$GT$7replace28_$u7b$$u7b$closure$u7d$$u7d$17hdc71cccb12095375E"(ptr align 1 %_1) unnamed_addr #1 {
start:
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h102d65dbfa674afeE(ptr align 1 @alloc_a500d906b91607583596fa15e63c2ada, i64 40, ptr align 8 @alloc_04b0b09a3a09fd0fa014e37514eae431) #19
  unreachable
}

; exec_v1::Array<T>::length
; Function Attrs: nonlazybind uwtable
define internal i64 @"_ZN7exec_v114Array$LT$T$GT$6length17h1a239d93c98ec070E"(ptr align 8 %self) unnamed_addr #0 {
start:
; call alloc::vec::Vec<T,A>::len
  %_0 = call i64 @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17hb47e093ab52f167eE"(ptr align 8 %self)
  ret i64 %_0
}

; exec_v1::Array<T>::new
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN7exec_v114Array$LT$T$GT$3new17hd69e1b20c62e132bE"(ptr sret([24 x i8]) align 8 %_0, ptr align 8 %data) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %self.i4 = alloca [8 x i8], align 4
  %self.i = alloca [8 x i8], align 4
  %0 = alloca [16 x i8], align 8
  %_27 = alloca [1 x i8], align 1
  %_24 = alloca [24 x i8], align 8
  %i = alloca [8 x i8], align 8
  %ptrs = alloca [24 x i8], align 8
  %data_rev = alloca [24 x i8], align 8
  store i8 0, ptr %_27, align 1
; invoke alloc::vec::Vec<T>::new
  invoke void @"_ZN5alloc3vec12Vec$LT$T$GT$3new17h1c991167e67589d4E"(ptr sret([24 x i8]) align 8 %data_rev)
          to label %bb26 unwind label %cleanup

bb22:                                             ; preds = %bb21, %cleanup
; invoke core::ptr::drop_in_place<alloc::vec::Vec<i32>>
  invoke void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$i32$GT$$GT$17hbed7f08e1a0bbd00E"(ptr align 8 %data) #21
          to label %bb23 unwind label %terminate

cleanup:                                          ; preds = %bb17, %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb22

bb26:                                             ; preds = %start
  br label %bb1

bb1:                                              ; preds = %bb27, %bb26
; invoke alloc::vec::Vec<T,A>::len
  %_4 = invoke i64 @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17hb2a5a87e59cc4172E"(ptr align 8 %data)
          to label %bb2 unwind label %cleanup1

bb21:                                             ; preds = %bb24, %bb25, %cleanup1
; invoke core::ptr::drop_in_place<alloc::vec::Vec<i32>>
  invoke void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$i32$GT$$GT$17hbed7f08e1a0bbd00E"(ptr align 8 %data_rev) #21
          to label %bb22 unwind label %terminate

cleanup1:                                         ; preds = %bb2.i, %bb5, %bb3, %bb6, %bb1
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
  store ptr %6, ptr %0, align 8
  %8 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %7, ptr %8, align 8
  br label %bb21

bb2:                                              ; preds = %bb1
  %_3 = icmp ugt i64 %_4, 0
  br i1 %_3, label %bb3, label %bb6

bb6:                                              ; preds = %bb2
; invoke alloc::vec::Vec<T>::new
  invoke void @"_ZN5alloc3vec12Vec$LT$T$GT$3new17h45015e188983f172E"(ptr sret([24 x i8]) align 8 %ptrs)
          to label %bb7 unwind label %cleanup1

bb3:                                              ; preds = %bb2
; invoke alloc::vec::Vec<T,A>::pop
  %9 = invoke { i32, i32 } @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3pop17h300ede68afa62c2aE"(ptr align 8 %data)
          to label %bb4 unwind label %cleanup1

bb7:                                              ; preds = %bb6
  store i8 1, ptr %_27, align 1
  store i64 0, ptr %i, align 8
  br label %bb8

bb8:                                              ; preds = %bb15, %bb7
; invoke alloc::vec::Vec<T,A>::len
  %_14 = invoke i64 @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17hb2a5a87e59cc4172E"(ptr align 8 %data_rev)
          to label %bb9 unwind label %cleanup2

bb25:                                             ; preds = %bb20, %cleanup2
  %10 = load i8, ptr %_27, align 1
  %11 = trunc i8 %10 to i1
  br i1 %11, label %bb24, label %bb21

cleanup2:                                         ; preds = %bb2.i7, %panic, %bb13, %bb12, %bb10, %bb8
  %12 = landingpad { ptr, i32 }
          cleanup
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
  store ptr %13, ptr %0, align 8
  %15 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %14, ptr %15, align 8
  br label %bb25

bb9:                                              ; preds = %bb8
  %_13 = icmp ugt i64 %_14, 0
  br i1 %_13, label %bb10, label %bb16

bb16:                                             ; preds = %bb9
  store i8 0, ptr %_27, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_24, ptr align 8 %ptrs, i64 24, i1 false)
  br label %bb17

bb10:                                             ; preds = %bb9
; invoke alloc::vec::Vec<T,A>::pop
  %16 = invoke { i32, i32 } @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$3pop17h300ede68afa62c2aE"(ptr align 8 %data_rev)
          to label %bb11 unwind label %cleanup2

bb20:                                             ; preds = %cleanup3
; invoke core::ptr::drop_in_place<alloc::vec::Vec<vstd::simple_pptr::PPtr<i32>>>
  invoke void @"_ZN4core3ptr78drop_in_place$LT$alloc..vec..Vec$LT$vstd..simple_pptr..PPtr$LT$i32$GT$$GT$$GT$17h3990e745265f84d6E"(ptr align 8 %_24) #21
          to label %bb25 unwind label %terminate

cleanup3:                                         ; No predecessors!
  %17 = landingpad { ptr, i32 }
          cleanup
  %18 = extractvalue { ptr, i32 } %17, 0
  %19 = extractvalue { ptr, i32 } %17, 1
  store ptr %18, ptr %0, align 8
  %20 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %19, ptr %20, align 8
  br label %bb20

bb17:                                             ; preds = %bb16
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_24, i64 24, i1 false)
  store i8 0, ptr %_27, align 1
; invoke core::ptr::drop_in_place<alloc::vec::Vec<i32>>
  invoke void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$i32$GT$$GT$17hbed7f08e1a0bbd00E"(ptr align 8 %data_rev)
          to label %bb18 unwind label %cleanup

bb18:                                             ; preds = %bb17
; call core::ptr::drop_in_place<alloc::vec::Vec<i32>>
  call void @"_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$i32$GT$$GT$17hbed7f08e1a0bbd00E"(ptr align 8 %data)
  ret void

terminate:                                        ; preds = %bb22, %bb21, %bb24, %bb20
  %21 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %22 = extractvalue { ptr, i32 } %21, 0
  %23 = extractvalue { ptr, i32 } %21, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17hb6fcb0ed7ad330b7E() #20
  unreachable

bb11:                                             ; preds = %bb10
  %_17.0 = extractvalue { i32, i32 } %16, 0
  %_17.1 = extractvalue { i32, i32 } %16, 1
  store i32 %_17.0, ptr %self.i4, align 4
  %24 = getelementptr inbounds i8, ptr %self.i4, i64 4
  store i32 %_17.1, ptr %24, align 4
  %25 = load i32, ptr %self.i4, align 4
  %_2.i5 = zext i32 %25 to i64
  %26 = icmp eq i64 %_2.i5, 0
  br i1 %26, label %bb2.i7, label %"_ZN4core6option15Option$LT$T$GT$6unwrap17hbb80375dd472d0cfE.exit9"

bb2.i7:                                           ; preds = %bb11
; invoke core::option::unwrap_failed
  invoke void @_ZN4core6option13unwrap_failed17hfd32652cc6017653E(ptr align 8 @alloc_f480ca867be60856403fe23f61a0153e) #19
          to label %.noexc8 unwind label %cleanup2

.noexc8:                                          ; preds = %bb2.i7
  unreachable

"_ZN4core6option15Option$LT$T$GT$6unwrap17hbb80375dd472d0cfE.exit9": ; preds = %bb11
  %27 = getelementptr inbounds i8, ptr %self.i4, i64 4
  %val.i6 = load i32, ptr %27, align 4
  br label %bb12

bb12:                                             ; preds = %"_ZN4core6option15Option$LT$T$GT$6unwrap17hbb80375dd472d0cfE.exit9"
; invoke vstd::simple_pptr::PPtr<V>::new
  %_20 = invoke i64 @"_ZN4vstd11simple_pptr13PPtr$LT$V$GT$3new17h9a00104cb8ea0e9bE"(i32 %val.i6)
          to label %bb13 unwind label %cleanup2

bb13:                                             ; preds = %bb12
; invoke alloc::vec::Vec<T,A>::push
  invoke void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17hc005cf2537d82a12E"(ptr align 8 %ptrs, i64 %_20)
          to label %bb14 unwind label %cleanup2

bb14:                                             ; preds = %bb13
  %28 = load i64, ptr %i, align 8
  %29 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %28, i64 1)
  %_23.0 = extractvalue { i64, i1 } %29, 0
  %_23.1 = extractvalue { i64, i1 } %29, 1
  br i1 %_23.1, label %panic, label %bb15

bb15:                                             ; preds = %bb14
  store i64 %_23.0, ptr %i, align 8
  br label %bb8

panic:                                            ; preds = %bb14
; invoke core::panicking::panic_const::panic_const_add_overflow
  invoke void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h18f3cdaf73f2417dE(ptr align 8 @alloc_7e4f9bc508f529f41090b83847272315) #19
          to label %unreachable unwind label %cleanup2

unreachable:                                      ; preds = %panic
  unreachable

bb24:                                             ; preds = %bb25
; invoke core::ptr::drop_in_place<alloc::vec::Vec<vstd::simple_pptr::PPtr<i32>>>
  invoke void @"_ZN4core3ptr78drop_in_place$LT$alloc..vec..Vec$LT$vstd..simple_pptr..PPtr$LT$i32$GT$$GT$$GT$17h3990e745265f84d6E"(ptr align 8 %ptrs) #21
          to label %bb21 unwind label %terminate

bb4:                                              ; preds = %bb3
  %_9.0 = extractvalue { i32, i32 } %9, 0
  %_9.1 = extractvalue { i32, i32 } %9, 1
  store i32 %_9.0, ptr %self.i, align 4
  %30 = getelementptr inbounds i8, ptr %self.i, i64 4
  store i32 %_9.1, ptr %30, align 4
  %31 = load i32, ptr %self.i, align 4
  %_2.i = zext i32 %31 to i64
  %32 = icmp eq i64 %_2.i, 0
  br i1 %32, label %bb2.i, label %"_ZN4core6option15Option$LT$T$GT$6unwrap17hbb80375dd472d0cfE.exit"

bb2.i:                                            ; preds = %bb4
; invoke core::option::unwrap_failed
  invoke void @_ZN4core6option13unwrap_failed17hfd32652cc6017653E(ptr align 8 @alloc_4543aa952ce8e713eeed3b4e051eb8a8) #19
          to label %.noexc unwind label %cleanup1

.noexc:                                           ; preds = %bb2.i
  unreachable

"_ZN4core6option15Option$LT$T$GT$6unwrap17hbb80375dd472d0cfE.exit": ; preds = %bb4
  %33 = getelementptr inbounds i8, ptr %self.i, i64 4
  %val.i = load i32, ptr %33, align 4
  br label %bb5

bb5:                                              ; preds = %"_ZN4core6option15Option$LT$T$GT$6unwrap17hbb80375dd472d0cfE.exit"
; invoke alloc::vec::Vec<T,A>::push
  invoke void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17hc9dca40045e66215E"(ptr align 8 %data_rev, i32 %val.i)
          to label %bb27 unwind label %cleanup1

bb27:                                             ; preds = %bb5
  br label %bb1

bb23:                                             ; preds = %bb22
  %34 = load ptr, ptr %0, align 8
  %35 = getelementptr inbounds i8, ptr %0, i64 8
  %36 = load i32, ptr %35, align 8
  %37 = insertvalue { ptr, i32 } poison, ptr %34, 0
  %38 = insertvalue { ptr, i32 } %37, i32 %36, 1
  resume { ptr, i32 } %38
}

; exec_v1::Array<T>::new::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN7exec_v114Array$LT$T$GT$3new28_$u7b$$u7b$closure$u7d$$u7d$17h0e76fbff48ea8035E"(ptr align 1 %_1) unnamed_addr #1 {
start:
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h102d65dbfa674afeE(ptr align 1 @alloc_a500d906b91607583596fa15e63c2ada, i64 40, ptr align 8 @alloc_8aa1b7189a4d5a061da3bf8da9316d47) #19
  unreachable
}

; exec_v1::Array<T>::borrow
; Function Attrs: nonlazybind uwtable
define internal { i64, i64 } @"_ZN7exec_v114Array$LT$T$GT$6borrow17h3a63cb713ff6a6fbE"(ptr align 8 %self, i64 %i) unnamed_addr #0 {
start:
; call <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
  %_4 = call align 8 ptr @"_ZN81_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..index..Index$LT$I$GT$$GT$5index17h54e90fe376be51fbE"(ptr align 8 %self, i64 %i, ptr align 8 @alloc_f531d87564a7775be286359171635b59)
  %ptr = load i64, ptr %_4, align 8
  %0 = insertvalue { i64, i64 } poison, i64 %i, 0
  %1 = insertvalue { i64, i64 } %0, i64 %ptr, 1
  ret { i64, i64 } %1
}

; exec_v1::Array<T>::borrow::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN7exec_v114Array$LT$T$GT$6borrow28_$u7b$$u7b$closure$u7d$$u7d$17h52d181637889b400E"(ptr align 1 %_1) unnamed_addr #1 {
start:
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h102d65dbfa674afeE(ptr align 1 @alloc_a500d906b91607583596fa15e63c2ada, i64 40, ptr align 8 @alloc_2701731f110eb3a8ac3cb5a242045270) #19
  unreachable
}

; exec_v1::Array<T>::terminate
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN7exec_v114Array$LT$T$GT$9terminate17hae66000cfc0f8df0E"(ptr align 8 %self, i64 %b.0, i64 %b.1) unnamed_addr #0 {
start:
  ret void
}

; exec_v1::tests::f
; Function Attrs: nonlazybind uwtable
define internal void @_ZN7exec_v15tests1f17h95473cd642c557a4E() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %b1 = alloca [16 x i8], align 8
  %b0 = alloca [16 x i8], align 8
  %_2 = alloca [24 x i8], align 8
  %array = alloca [24 x i8], align 8
; call alloc::alloc::exchange_malloc
  %_6 = call ptr @_ZN5alloc5alloc15exchange_malloc17h1f290aac71a702e4E(i64 12, i64 4)
  %_30 = ptrtoint ptr %_6 to i64
  %_33 = and i64 %_30, 3
  %_34 = icmp eq i64 %_33, 0
  br i1 %_34, label %bb16, label %panic

bb16:                                             ; preds = %start
  %1 = getelementptr inbounds [3 x i32], ptr %_6, i64 0, i64 0
  store i32 0, ptr %1, align 4
  %2 = getelementptr inbounds [3 x i32], ptr %_6, i64 0, i64 1
  store i32 1, ptr %2, align 4
  %3 = getelementptr inbounds [3 x i32], ptr %_6, i64 0, i64 2
  store i32 2, ptr %3, align 4
; call alloc::slice::<impl [T]>::into_vec
  call void @"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17hd6b9e4db74e020bfE"(ptr sret([24 x i8]) align 8 %_2, ptr align 4 %_6, i64 3)
; call exec_v1::Array<T>::new
  call void @"_ZN7exec_v114Array$LT$T$GT$3new17hd69e1b20c62e132bE"(ptr sret([24 x i8]) align 8 %array, ptr align 8 %_2)
; invoke exec_v1::Array<T>::borrow
  %4 = invoke { i64, i64 } @"_ZN7exec_v114Array$LT$T$GT$6borrow17h3a63cb713ff6a6fbE"(ptr align 8 %array, i64 0)
          to label %bb4 unwind label %cleanup

panic:                                            ; preds = %start
; call core::panicking::panic_misaligned_pointer_dereference
  call void @_ZN4core9panicking36panic_misaligned_pointer_dereference17he691b8c469f405b1E(i64 4, i64 %_30, ptr align 8 @alloc_afbc7bc99cc873514471c85c8b876255) #18
  unreachable

bb14:                                             ; preds = %cleanup
; invoke core::ptr::drop_in_place<exec_v1::Array<i32>>
  invoke void @"_ZN4core3ptr46drop_in_place$LT$exec_v1..Array$LT$i32$GT$$GT$17hb37936d2bb4c5872E"(ptr align 8 %array) #21
          to label %bb15 unwind label %terminate

cleanup:                                          ; preds = %bb11, %bb10, %bb9, %bb8, %bb7, %bb6, %bb5, %bb4, %bb16
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
  store ptr %6, ptr %0, align 8
  %8 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %7, ptr %8, align 8
  br label %bb14

bb4:                                              ; preds = %bb16
  %9 = extractvalue { i64, i64 } %4, 0
  %10 = extractvalue { i64, i64 } %4, 1
  store i64 %9, ptr %b0, align 8
  %11 = getelementptr inbounds i8, ptr %b0, i64 8
  store i64 %10, ptr %11, align 8
; invoke exec_v1::Array<T>::borrow
  %12 = invoke { i64, i64 } @"_ZN7exec_v114Array$LT$T$GT$6borrow17h3a63cb713ff6a6fbE"(ptr align 8 %array, i64 1)
          to label %bb5 unwind label %cleanup

bb5:                                              ; preds = %bb4
  %13 = extractvalue { i64, i64 } %12, 0
  %14 = extractvalue { i64, i64 } %12, 1
  store i64 %13, ptr %b1, align 8
  %15 = getelementptr inbounds i8, ptr %b1, i64 8
  store i64 %14, ptr %15, align 8
; invoke exec_v1::Borrow<T>::read
  %b0_i = invoke align 4 ptr @"_ZN7exec_v115Borrow$LT$T$GT$4read17h4fb74ce01bd3bda2E"(ptr align 8 %b0)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
; invoke exec_v1::Borrow<T>::read
  %b0_ii = invoke align 4 ptr @"_ZN7exec_v115Borrow$LT$T$GT$4read17h4fb74ce01bd3bda2E"(ptr align 8 %b0)
          to label %bb7 unwind label %cleanup

bb7:                                              ; preds = %bb6
; invoke exec_v1::Borrow<T>::replace
  %_16 = invoke i32 @"_ZN7exec_v115Borrow$LT$T$GT$7replace17hdbf402f7ee34963bE"(ptr align 8 %b0, i32 66)
          to label %bb8 unwind label %cleanup

bb8:                                              ; preds = %bb7
; invoke exec_v1::Borrow<T>::replace
  %_18 = invoke i32 @"_ZN7exec_v115Borrow$LT$T$GT$7replace17hdbf402f7ee34963bE"(ptr align 8 %b1, i32 77)
          to label %bb9 unwind label %cleanup

bb9:                                              ; preds = %bb8
  %_22.0 = load i64, ptr %b1, align 8
  %16 = getelementptr inbounds i8, ptr %b1, i64 8
  %_22.1 = load i64, ptr %16, align 8
; invoke exec_v1::Array<T>::terminate
  invoke void @"_ZN7exec_v114Array$LT$T$GT$9terminate17hae66000cfc0f8df0E"(ptr align 8 %array, i64 %_22.0, i64 %_22.1)
          to label %bb10 unwind label %cleanup

bb10:                                             ; preds = %bb9
  %_25.0 = load i64, ptr %b0, align 8
  %17 = getelementptr inbounds i8, ptr %b0, i64 8
  %_25.1 = load i64, ptr %17, align 8
; invoke exec_v1::Array<T>::terminate
  invoke void @"_ZN7exec_v114Array$LT$T$GT$9terminate17hae66000cfc0f8df0E"(ptr align 8 %array, i64 %_25.0, i64 %_25.1)
          to label %bb11 unwind label %cleanup

bb11:                                             ; preds = %bb10
; invoke exec_v1::tests::print
  invoke void @_ZN7exec_v15tests5print17hc6bf05895b5c5ce5E(ptr align 8 %array)
          to label %bb12 unwind label %cleanup

bb12:                                             ; preds = %bb11
; call core::ptr::drop_in_place<exec_v1::Array<i32>>
  call void @"_ZN4core3ptr46drop_in_place$LT$exec_v1..Array$LT$i32$GT$$GT$17hb37936d2bb4c5872E"(ptr align 8 %array)
  ret void

terminate:                                        ; preds = %bb14
  %18 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %19 = extractvalue { ptr, i32 } %18, 0
  %20 = extractvalue { ptr, i32 } %18, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17hb6fcb0ed7ad330b7E() #20
  unreachable

bb15:                                             ; preds = %bb14
  %21 = load ptr, ptr %0, align 8
  %22 = getelementptr inbounds i8, ptr %0, i64 8
  %23 = load i32, ptr %22, align 8
  %24 = insertvalue { ptr, i32 } poison, ptr %21, 0
  %25 = insertvalue { ptr, i32 } %24, i32 %23, 1
  resume { ptr, i32 } %25
}

; exec_v1::tests::print
; Function Attrs: nonlazybind uwtable
define internal void @_ZN7exec_v15tests5print17hc6bf05895b5c5ce5E(ptr align 8 %a) unnamed_addr #0 {
start:
  %_3.i = alloca [16 x i8], align 8
  %_23 = alloca [48 x i8], align 8
  %_19 = alloca [8 x i8], align 8
  %_17 = alloca [16 x i8], align 8
  %_16 = alloca [16 x i8], align 8
  %_13 = alloca [48 x i8], align 8
  %b = alloca [16 x i8], align 8
  %_7 = alloca [16 x i8], align 8
  %iter = alloca [16 x i8], align 8
; call exec_v1::Array<T>::length
  %_4 = call i64 @"_ZN7exec_v114Array$LT$T$GT$6length17h1a239d93c98ec070E"(ptr align 8 %a)
; call <I as core::iter::traits::collect::IntoIterator>::into_iter
  %0 = call { i64, i64 } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h668e2690d8919983E"(i64 0, i64 %_4)
  %_2.0 = extractvalue { i64, i64 } %0, 0
  %_2.1 = extractvalue { i64, i64 } %0, 1
  store i64 %_2.0, ptr %iter, align 8
  %1 = getelementptr inbounds i8, ptr %iter, i64 8
  store i64 %_2.1, ptr %1, align 8
  br label %bb3

bb3:                                              ; preds = %bb6, %start
; call core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next
  %2 = call { i64, i64 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h18636b3d4ab8910bE"(ptr align 8 %iter)
  %3 = extractvalue { i64, i64 } %2, 0
  %4 = extractvalue { i64, i64 } %2, 1
  store i64 %3, ptr %_7, align 8
  %5 = getelementptr inbounds i8, ptr %_7, i64 8
  store i64 %4, ptr %5, align 8
  %_9 = load i64, ptr %_7, align 8
  %6 = icmp eq i64 %_9, 0
  br i1 %6, label %bb7, label %bb6

bb7:                                              ; preds = %bb3
; call core::fmt::Arguments::new_const
  call void @_ZN4core3fmt9Arguments9new_const17h8de8285d1e415f0fE(ptr sret([48 x i8]) align 8 %_23, ptr align 8 @alloc_905721c3cf206ce86b0d110014b92e9a)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17hd6837e34a66547ddE(ptr align 8 %_23)
  ret void

bb6:                                              ; preds = %bb3
  %7 = getelementptr inbounds i8, ptr %_7, i64 8
  %i = load i64, ptr %7, align 8
; call exec_v1::Array<T>::borrow
  %8 = call { i64, i64 } @"_ZN7exec_v114Array$LT$T$GT$6borrow17h3a63cb713ff6a6fbE"(ptr align 8 %a, i64 %i)
  %9 = extractvalue { i64, i64 } %8, 0
  %10 = extractvalue { i64, i64 } %8, 1
  store i64 %9, ptr %b, align 8
  %11 = getelementptr inbounds i8, ptr %b, i64 8
  store i64 %10, ptr %11, align 8
; call exec_v1::Borrow<T>::read
  %12 = call align 4 ptr @"_ZN7exec_v115Borrow$LT$T$GT$4read17h4fb74ce01bd3bda2E"(ptr align 8 %b)
  store ptr %12, ptr %_19, align 8
  store ptr %_19, ptr %_3.i, align 8
  %13 = getelementptr inbounds i8, ptr %_3.i, i64 8
  store ptr @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hed6ec812c9f48b7dE", ptr %13, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_17, ptr align 8 %_3.i, i64 16, i1 false)
  %14 = getelementptr inbounds [1 x %"core::fmt::rt::Argument<'_>"], ptr %_16, i64 0, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %14, ptr align 8 %_17, i64 16, i1 false)
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hbad76aae779359c6E(ptr sret([48 x i8]) align 8 %_13, ptr align 8 @alloc_4e0023beeca5f8e5d06a41f60e7c1e6e, ptr align 8 %_16)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17hd6837e34a66547ddE(ptr align 8 %_13)
  %15 = load i64, ptr %b, align 8
  %16 = getelementptr inbounds i8, ptr %b, i64 8
  %17 = load i64, ptr %16, align 8
; call exec_v1::Array<T>::terminate
  call void @"_ZN7exec_v114Array$LT$T$GT$9terminate17hae66000cfc0f8df0E"(ptr align 8 %a, i64 %15, i64 %17)
  br label %bb3

bb5:                                              ; No predecessors!
  unreachable
}

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h4d90db0530245041E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #4

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctpop.i64(i64) #5

; core::panicking::panic_nounwind
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking14panic_nounwind17h9f485ff9b02bac75E(ptr align 1, i64) unnamed_addr #6

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17h3eea515d05f7a35eE(ptr align 8, ptr align 8) unnamed_addr #7

; core::panicking::panic_cannot_unwind
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking19panic_cannot_unwind17hea865182d7ce50afE() unnamed_addr #6

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #8

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he44de31354db2158E"(ptr align 4, ptr align 8) unnamed_addr #0

; core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17ha980ac9704f347d8E"(ptr align 4, ptr align 8) unnamed_addr #0

; core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h5c1cc86791bc1522E"(ptr align 4, ptr align 8) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #5

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #5

; core::panicking::panic_in_cleanup
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17hb6fcb0ed7ad330b7E() unnamed_addr #6

; core::alloc::layout::Layout::is_size_align_valid
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core5alloc6layout6Layout19is_size_align_valid17h27157fff07002cf3E(i64, i64) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #9

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h29d33a10d2cc93f0E(ptr align 8) unnamed_addr #7

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core6option13unwrap_failed17hfd32652cc6017653E(ptr align 8) unnamed_addr #7

; vstd::raw_ptr::allocate
; Function Attrs: nonlazybind uwtable
declare ptr @_ZN4vstd7raw_ptr8allocate17haf43c87912e14848E(i64, i64) unnamed_addr #0

; alloc::alloc::handle_alloc_error
; Function Attrs: cold noreturn nonlazybind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h6235a660a5e8e3a6E(i64, i64) unnamed_addr #10

; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc(i64, i64 allocalign) unnamed_addr #11

; Function Attrs: nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias ptr @__rust_alloc_zeroed(i64, i64 allocalign) unnamed_addr #12

; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr, i64, i64) unnamed_addr #13

; Function Attrs: nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias ptr @__rust_realloc(ptr allocptr, i64, i64 allocalign, i64) unnamed_addr #14

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #15

; alloc::raw_vec::handle_error
; Function Attrs: cold noreturn nonlazybind uwtable
declare void @_ZN5alloc7raw_vec12handle_error17he4316ba2e8167751E(i64, i64) unnamed_addr #10

; core::panicking::panic_bounds_check
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking18panic_bounds_check17hab02a8df06d3a143E(i64, i64, ptr align 8) unnamed_addr #7

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h102d65dbfa674afeE(ptr align 1, i64, ptr align 8) unnamed_addr #7

; core::panicking::panic_const::panic_const_add_overflow
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h18f3cdaf73f2417dE(ptr align 8) unnamed_addr #7

; core::panicking::panic_misaligned_pointer_dereference
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking36panic_misaligned_pointer_dereference17he691b8c469f405b1E(i64, i64, ptr align 8) unnamed_addr #6

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17hd6837e34a66547ddE(ptr align 8) unnamed_addr #0

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #16 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h8eae89d86affba27E(ptr @_ZN7exec_v14main17hd1092b66ca4bfeb0E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { inlinehint nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #5 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #6 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #7 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #8 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #10 = { cold noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #11 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #12 = { nounwind nonlazybind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #13 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #14 = { nounwind nonlazybind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #15 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #16 = { nonlazybind "target-cpu"="x86-64" }
attributes #17 = { nounwind }
attributes #18 = { noreturn nounwind }
attributes #19 = { noreturn }
attributes #20 = { cold noreturn nounwind }
attributes #21 = { cold }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{!"rustc version 1.82.0 (f6e511eec 2024-10-15)"}
!4 = !{i32 3951088}
