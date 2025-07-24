#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface RCT_EXTERN_MODULE(RNPinnipedEditor, RCTEventEmitter)

// Editor lifecycle
RCT_EXTERN_METHOD(createEditor:(NSString *)editorId
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)

RCT_EXTERN_METHOD(destroyEditor:(NSString *)editorId
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)

// Document operations
RCT_EXTERN_METHOD(parseMarkdown:(NSString *)editorId
                  markdown:(NSString *)markdown
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)

RCT_EXTERN_METHOD(toMarkdown:(NSString *)editorId
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)

// Table operations
RCT_EXTERN_METHOD(navigateTable:(NSString *)editorId
                  blockIndex:(NSNumber *)blockIndex
                  currentRow:(NSNumber *)currentRow
                  currentCol:(NSNumber *)currentCol
                  direction:(NSString *)direction
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)

RCT_EXTERN_METHOD(getTableCell:(NSString *)editorId
                  blockIndex:(NSNumber *)blockIndex
                  row:(NSNumber *)row
                  col:(NSNumber *)col
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)

// Utility methods
RCT_EXTERN_METHOD(getDocumentInfo:(NSString *)editorId
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)

@end