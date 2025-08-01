# Yaak 修改记录 - License & Update 移除

## 🎯 修改概述

本文档记录了为构建无 License 验证、无自动更新检查的 Yaak Mac 安装包所做的所有代码修改。这些修改旨在：

1. **完全绕过 License 验证系统**
2. **移除所有 License 相关的前端界面元素**
3. **禁用自动更新检查功能**
4. **移除所有更新相关的前端界面元素**

---

## 后端修改 (Rust)

### 1. License 验证系统修改

#### 文件: `src-tauri/yaak-license/src/license.rs`

**修改目标**: 完全绕过 License 验证逻辑

**关键修改**:

```rust
// 始终返回商业版本状态，绕过所有验证
pub async fn check_license<R: Runtime>(_window: &WebviewWindow<R>) -> Result<LicenseCheckStatus> {
    // Always return CommercialUse to bypass all license checks
    Ok(LicenseCheckStatus::CommercialUse)
}

// 仅使用本地假激活 ID，不发送网络请求
pub async fn activate_license<R: Runtime>(
    window: &WebviewWindow<R>,
    license_key: &str,
) -> Result<()> {
    // Store a fake activation ID to simulate activation
    let fake_activation_id = format!("local-{}", license_key.to_string().chars().take(8).collect::<String>());
    
    window.app_handle().db().set_key_value_string(
        KV_ACTIVATION_ID_KEY,
        KV_NAMESPACE,
        &fake_activation_id,
        &UpdateSource::from_window(&window),
    );

    if let Err(e) = window.emit("license-activated", true) {
        warn!("Failed to emit check-license event: {}", e);
    }

    Ok(())
}

// 仅删除本地激活 ID，不发送网络请求
pub async fn deactivate_license<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    let app_handle = window.app_handle();

    // Simply remove the local activation ID without network request
    app_handle.db().delete_key_value(
        KV_ACTIVATION_ID_KEY,
        KV_NAMESPACE,
        &UpdateSource::from_window(&window),
    )?;

    if let Err(e) = app_handle.emit("license-deactivated", true) {
        warn!("Failed to emit deactivate-license event: {}", e);
    }

    Ok(())
}
```

**修改影响**:
- `check_license()`: 不再进行任何网络验证，始终返回已激活的商业版本状态
- `activate_license()`: 不再发送激活请求到服务器，仅在本地生成假的激活 ID
- `deactivate_license()`: 不再发送注销请求到服务器，仅删除本地激活 ID

### 2. 自动更新检查禁用

#### 文件: `src-tauri/src/lib.rs`

**修改目标**: 禁用窗口焦点时的自动更新检查

**原始代码**:
```rust
WindowEvent::Focused(focused) => {
    if *focused {
        // Check for updates when the window gains focus
        check_for_updates_cmd(window.clone()).await;
    }
}
```

**修改后**:
```rust
// 禁用自动更新检查
WindowEvent::Focused(_focused) => {
    // Automatic update checking disabled
    // Original code: check_for_updates_cmd(window.clone()).await;
}
```

**修改影响**:
- 应用窗口获得焦点时不再自动检查更新
- 避免弹出 "Update Available" 对话框

---

## 🎨 前端修改 (TypeScript/React)

### 1. WorkspaceHeader 组件修改

#### 文件: `src-web/components/WorkspaceHeader.tsx`

**修改目标**: 移除 License Badge 显示

```typescript
// 🗑️ 已移除：import { LicenseBadge } from './LicenseBadge';
// 🗑️ 已移除：<LicenseBadge />
```

### 2. Settings 组件修改

#### 文件: `src-web/components/Settings/Settings.tsx`

**修改目标**: 移除 License 标签页

```typescript
// 🗑️ 已移除：import { SettingsLicense } from './SettingsLicense';
// 🗑️ 已移除：const TAB_LICENSE = 'license';
// 🗑️ 已移除：License 标签页和相关渲染逻辑
```

### 3. SettingsDropdown 组件修改

#### 文件: `src-web/components/SettingsDropdown.tsx`

**修改目标**: 移除 "Purchase License" 和 "Check for Updates" 选项

```typescript
// 🗑️ 已移除：import { useCheckForUpdates } from '../hooks/useCheckForUpdates';
// 🗑️ 已移除："Purchase License" 菜单项
// 🗑️ 已移除："Check for Updates" 菜单项
```

### 4. SettingsGeneral 组件修改

#### 文件: `src-web/components/Settings/SettingsGeneral.tsx`

**修改目标**: 移除 "Update Channel" 设置和 "Check for Updates" 按钮

```typescript
// 🗑️ 已移除：import { useCheckForUpdates } from '../../hooks/useCheckForUpdates';
// 🗑️ 已移除："Update Channel" 设置区块
// 🗑️ 已移除："Check for Updates" 按钮
```

---

## ⚙️ 构建配置修改

### 1. 本地构建配置

#### 文件: `src-tauri/tauri-local.conf.json` (新增)

**修改目标**: 创建本地构建配置，绕过代码签名

```json
{
  "bundle": {
    "active": true,
    "targets": ["dmg"],
    "identifier": "com.yaak.app.local",
    "macOS": {
      "signingIdentity": null
    },
    "externalBin": [
      "vendored/node/yaaknode"
    ],
    "resources": [
      "vendored/plugin-runtime",
      "vendored/plugins"
    ]
  },
  "productName": "Yaak",
  "version": "0.0.0"
}
```

**重要说明**: `externalBin` 路径必须指向 `vendored/node/yaaknode`，这样 Tauri 才能找到通过 bootstrap 下载的 Node.js 运行时。

### 2. Package.json 脚本修改

#### 文件: `package.json`

**新增构建脚本**:
```json
{
  "scripts": {
    "app-build-local": "tauri build --config ./src-tauri/tauri-local.conf.json"
  }
}
```

---

## ⚡ 构建使用说明

### 构建命令
```bash
# 首次构建前必须运行 bootstrap (下载依赖)
npm run bootstrap

# 本地无签名构建 (推荐)
npm run app-build-local

# 标准构建 (需要代码签名)
npm run app-build
```

### 构建产物
- **DMG 文件**: `src-tauri/target/release/bundle/dmg/Yaak_0.0.0_aarch64.dmg` (约 54MB)
- **App 文件**: `src-tauri/target/release/bundle/macos/Yaak.app`

### 构建注意事项
1. **首次构建**: 必须先运行 `npm run bootstrap` 下载 Node.js 运行时和 protoc
2. **Node.js 运行时**: 系统会自动下载并放置到 `src-tauri/vendored/node/yaaknode-aarch64-apple-darwin`
3. **插件系统**: 所有插件会被构建并放置到 `src-tauri/vendored/plugins/`
4. **构建警告**: 关于未使用的更新相关常量的警告是正常的（因为我们禁用了更新功能）

---

## 🚨 代码合并注意事项

### 必须保留的本地修改
```bash
# 这些修改在合并时必须保留，否则会重新启用 License 验证
src-tauri/yaak-license/src/license.rs   # License 绕过逻辑
src-tauri/src/lib.rs                     # 自动更新禁用
src-web/components/WorkspaceHeader.tsx   # License Badge 移除
src-web/components/Settings/Settings.tsx # License 标签页移除
src-web/components/SettingsDropdown.tsx  # License/更新菜单移除
src-web/components/Settings/SettingsGeneral.tsx # 更新设置移除
```

### 冲突处理优先级

**高优先级保留 (本地修改)**:
- `src-tauri/yaak-license/src/license.rs` 中的绕过逻辑
- `src-tauri/src/lib.rs` 中的自动更新禁用
- 所有前端 License/更新相关界面的移除

**可接受更新**:
- 其他功能性代码改进
- 依赖项更新
- 新功能添加 (只要不重新引入 License 验证)
- Bug 修复
- 性能优化

### 关键函数监控
合并时特别注意以下函数的变化：

**后端**:
- `check_license()` - 确保始终返回 `CommercialUse`
- `activate_license()` - 确保不发送网络请求
- `deactivate_license()` - 确保不发送网络请求
- `WindowEvent::Focused` 处理 - 确保不自动检查更新

**前端**:
- `WorkspaceHeader` 组件 - 确保不包含 `LicenseBadge`
- `Settings` 组件 - 确保不包含 License 标签页
- `SettingsDropdown` 组件 - 确保不包含 License/更新菜单项
- `SettingsGeneral` 组件 - 确保不包含更新设置

### 新增功能处理
如果源仓库新增了与 License 或更新相关的功能：
1. **忽略新的 License 验证逻辑**
2. **移除新的 License 相关 UI 组件**
3. **禁用新的自动更新机制**
4. **移除新的更新相关 UI 元素**

---

## ✅ 验证清单

合并代码后，确保以下功能正常：

### License 系统验证
- [ ] 应用启动后直接进入工作界面，无 License 弹窗
- [ ] 工作区头部没有 License Badge 显示
- [ ] 设置页面没有 License 标签页
- [ ] 下拉菜单没有 "Purchase License" 选项
- [ ] 后台始终识别为已激活的商业版本

### 更新系统验证
- [ ] 应用获得焦点时不弹出 "Update Available" 对话框
- [ ] 下拉菜单没有 "Check for Updates" 选项
- [ ] 设置页面没有 "Update Channel" 配置
- [ ] 设置页面没有 "Check for Updates" 按钮

### 核心功能验证
- [ ] API 客户端功能正常 (REST, GraphQL, gRPC, WebSocket, SSE)
- [ ] 插件系统正常工作
- [ ] 所有商业版本功能可用
- [ ] 应用可以正常构建为 DMG 包

### 构建测试
```bash
# 验证构建流程
npm run app-build-local
# 确认产物生成：src-tauri/target/release/bundle/dmg/Yaak_0.0.0_aarch64.dmg
```

---

## 🔄 后续合并流程建议

1. **拉取上游更新前**：
   ```bash
   git stash  # 暂存当前修改
   git pull upstream main  # 拉取上游更新
   ```

2. **处理合并冲突时**：
   - 参考"必须保留的本地修改"清单
   - 确保关键的绕过逻辑不被覆盖

3. **合并完成后**：
   - 按照"验证清单"进行快速测试
   - 运行 `npm run bootstrap` 下载必要的依赖
   - 运行 `npm run app-build-local` 确保构建正常

4. **如果遇到新的 License/更新功能**：
   - 参考本文档中的修改模式
   - 对新增的相关代码进行相同的绕过/移除处理

---

## 📋 修改文件完整清单

### 后端文件 (Rust)
1. ✅ `src-tauri/yaak-license/src/license.rs` - License 验证绕过
2. ✅ `src-tauri/yaak-license/src/commands.rs` - License 命令接口 (保持不变)
3. ✅ `src-tauri/src/lib.rs` - 禁用自动更新检查

### 前端文件 (TypeScript/React)
1. ✅ `src-web/components/WorkspaceHeader.tsx` - 移除 License Badge
2. ✅ `src-web/components/Settings/Settings.tsx` - 移除 License 标签页
3. ✅ `src-web/components/SettingsDropdown.tsx` - 移除 License 和更新菜单项
4. ✅ `src-web/components/Settings/SettingsGeneral.tsx` - 移除更新设置和按钮

### 配置文件
1. ✅ `src-tauri/tauri-local.conf.json` - 本地构建配置 (新增，包含正确的 externalBin 路径)
2. ✅ `package.json` - 新增本地构建脚本

---

## 🏁 最终构建结果

**✅ 构建成功完成！**

- **DMG 文件**: `src-tauri/target/release/bundle/dmg/Yaak_0.0.0_aarch64.dmg` (54MB)
- **构建时间**: 约 1-2 分钟 (包含前端和后端编译)
- **支持架构**: Apple Silicon (aarch64-apple-darwin)
- **签名状态**: 无签名 (适合本地使用)

**用户体验**:
- ✅ 启动后直接进入工作界面，无任何弹窗
- ✅ 完全没有 License 相关界面元素
- ✅ 完全没有自动更新检查和相关按钮
- ✅ 所有 API 客户端功能正常工作 (REST, GraphQL, gRPC, WebSocket, SSE)
- ✅ 插件系统正常工作
- ✅ 识别为已激活的商业版本

---

**⚠️ 重要提醒**: 此修改仅用于学习和内部测试，请遵守原项目的开源协议。

**文档版本**: v2.0  
**创建时间**: 2024年8月1日  
**适用版本**: Yaak 0.0.0 (基于 mountain-loop/yaak)  
**修改目标**: 完全移除 License 验证和自动更新检查
