// libnode.cpp - Node.js 绑定实现（供 napi 使用）
// 这个文件是可选的，如果只需要 Tauri 绑定可以删除

#include <napi.h>

// 导出 GardenNative 构造函数
Napi::Object GardenNative::Init(Napi::Env env, Napi::Object exports) {
  Napi::Function func = DefineClass(env, "GardenNative", {
    GardenNative::NewMethod,
    GardenNative::PlantMethod,
    GardenNative::WaterMethod,
    GardenNative::SunMethod,
    GardenNative::HarvestMethod,
    GardenNative::CalculateGrowthMethod,
    GardenNative::VariantRollMethod,
    GardenNative::CrossCheckMethod,
    GardenNative::CheckStatusMethod,
  });

  exports.Set("GardenNative", func);
  return exports;
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  return GardenNative::Init(env, exports);
}

NODE_MODULE(NODE_GYP_MODULE_NAME, Init)
