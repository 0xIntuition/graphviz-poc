//
//  HelloWorldViewController.m
//  intuition_mobile
//
//  Created by Simonas Karuzas on 11/04/2024.
//

#import "HelloWorldViewController.h"

@interface HelloWorldViewController ()

@end

@implementation HelloWorldViewController

- (void)viewDidLoad {
    [super viewDidLoad];

    self.view.backgroundColor = [UIColor systemBackgroundColor];
    UIButton *closeButton = [UIButton buttonWithType:UIButtonTypeSystem];
    [closeButton setTitle:@"Close" forState:UIControlStateNormal];
    closeButton.frame = CGRectMake(20, 50, 100, 50); // Adjust frame as needed
    [closeButton addTarget:self
                    action:@selector(closeViewController)
          forControlEvents:UIControlEventTouchUpInside];
    [self.view addSubview:closeButton];
}

- (void)closeViewController {
    [self dismissViewControllerAnimated:YES completion:nil];
}



@end
