# Custom React Native mobile app

Currently broken

```
*** Terminating app due to uncaught exception 'NSInvalidArgumentException', reason: '-[WinitApplicationDelegate window]: unrecognized selector sent to instance 0x600000031a20'
*** First throw call stack:
(
	0   CoreFoundation                      0x0000000180491128 __exceptionPreprocess + 172
	1   libobjc.A.dylib                     0x000000018008412c objc_exception_throw + 56
	2   CoreFoundation                      0x00000001804a5f78 +[NSObject(NSObject) instanceMethodSignatureForSelector:] + 0
	3   CoreFoundation                      0x0000000180495278 ___forwarding___ + 1280
	4   CoreFoundation                      0x000000018049759c _CF_forwarding_prep_0 + 92
	5   intuition_mobile                    0x0000000100ce6b4c -[RCTAppearance init] + 176
	6   intuition_mobile                    0x0000000100c52bc4 __115-[RCTModuleData initWithModuleClass:bridge:moduleRegistry:viewRegistry_DEPRECATED:bundleManager:callableJSModules:]_block_invoke + 36
	7   intuition_mobile                    0x0000000100c53664 -[RCTModuleData setUpInstanceAndBridge:] + 1324
	8   intuition_mobile                    0x0000000100c55684 __25-[RCTModuleData instance]_block_invoke + 44
	9   intuition_mobile                    0x0000000100cb6b30 RCTUnsafeExecuteOnMainQueueSync + 52
	10  intuition_mobile                    0x0000000100c552ec -[RCTModuleData instance] + 816
	11  intuition_mobile                    0x0000000100c01a70 __49-[RCTCxxBridge _prepareModulesWithDispatchGroup:]_block_invoke + 160
	12  libdispatch.dylib                   0x000000010f6e80f0 _dispatch_call_block_and_release + 24
	13  libdispatch.dylib                   0x000000010f6e993c _dispatch_client_callout + 16
	14  libdispatch.dylib                   0x000000010f6f96ac _dispatch_main_queue_drain + 1428
	15  libdispatch.dylib                   0x000000010f6f9108 _dispatch_main_queue_callback_4CF + 40
	16  CoreFoundation                      0x00000001803f1a30 __CFRUNLOOP_IS_SERVICING_THE_MAIN_DISPATCH_QUEUE__ + 12
	17  CoreFoundation                      0x00000001803ec148 __CFRunLoopRun + 1936
	18  CoreFoundation                      0x00000001803eb5a4 CFRunLoopRunSpecific + 572
	19  GraphicsServices                    0x000000018e9fbae4 GSEventRunModal + 160
	20  UIKitCore                           0x00000001852f02e4 -[UIApplication _run] + 868
	21  UIKitCore                           0x00000001852f3f5c UIApplicationMain + 124
	22  intuition_mobile                    0x0000000101445f00 _ZN5winit13platform_impl8platform10event_loop18EventLoop$LT$T$GT$3run17h18607c39e920d9daE + 696
	23  intuition_mobile                    0x00000001014275a4 _ZN5winit10event_loop25EventLoopBuilder$LT$T$GT$15with_user_event17hd58e0c6437ab8a2eE + 0
	24  intuition_mobile                    0x0000000101469848 _ZN10bevy_winit12winit_runner17h78d2e9d6c0ec531cE + 1672
	25  intuition_mobile                    0x000000010144038c _ZN4core3ops8function6FnOnce9call_once17h2efe0317a6aa47ebE + 48
	26  intuition_mobile                    0x0000000101440128 _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17he156fad607c82562E + 56
	27  intuition_mobile                    0x00000001049e5130 _ZN90_$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$9call_once17h58ef3cfc283de613E + 100
	28  intuition_mobile                    0x00000001049d5960 _ZN8bevy_app3app3App3run17hc22f75106857e251E + 348
	29  intuition_mobile                    0x00000001012add80 _ZN16intuition_mobile4main17ha234469b31eaa945E + 720
	30  intuition_mobile                    0x00000001012adaa8 main_rs + 12
	31  intuition_mobile                    0x00000001009237d0 main + 28
	32  dyld                                0x000000010e119544 start_sim + 20
	33  ???                                 0x000000010e3760e0 0x0 + 4533477600
	34  ???                                 0xcc6a000000000000 0x0 + 14729585531268628480
)
libc++abi: terminating due to uncaught exception of type NSException 
```
