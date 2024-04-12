//
//  HelloWorldViewController.m
//  intuition_mobile
//
//  Created by Simonas Karuzas on 11/04/2024.
//

#import "HelloWorldViewController.h"
#import <React/RCTRootView.h>

@interface HelloWorldViewController ()

@end

@implementation HelloWorldViewController

- (void)viewDidLoad {
    [super viewDidLoad];

    self.view.backgroundColor = [UIColor systemBackgroundColor];
    NSURL *jsCodeLocation = [NSURL URLWithString:@"http://localhost:8081/index.bundle?platform=ios"];

    RCTRootView *rootView =
      [[RCTRootView alloc] initWithBundleURL: jsCodeLocation
                                  moduleName: @"RNHighScores"
                           initialProperties:
                             @{
                               @"scores" : @[
                                 @{
                                   @"name" : @"Alex",
                                   @"value": @"42"
                                  },
                                 @{
                                   @"name" : @"Joel",
                                   @"value": @"10"
                                 }
                               ]
                             }
                               launchOptions: nil];
    //[self.view addSubview:rootView];
    self.view = rootView;
}

- (void)closeViewController {
    [self dismissViewControllerAnimated:YES completion:nil];
}



@end
