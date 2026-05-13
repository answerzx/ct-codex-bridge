pub const INDEX_HTML: &str = r##"<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Codex 账号桥接面板</title>
  <link rel="icon" type="image/svg+xml" href="/favicon.svg">
  <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
  <meta name="theme-color" content="#edf6ff">
  <meta name="application-name" content="Codex Bridge">
  <meta name="apple-mobile-web-app-title" content="Codex Bridge">
  <style>
    :root {
      color-scheme: light;
      --page: #edf6ff;
      --surface: rgba(255, 255, 255, 0.72);
      --surface-solid: #ffffff;
      --surface-soft: rgba(255, 255, 255, 0.54);
      --blue-panel: rgba(245, 250, 255, 0.9);
      --blue-panel-soft: rgba(232, 244, 255, 0.68);
      --blue-border: rgba(0, 113, 227, 0.18);
      --text: #1d1d1f;
      --secondary: #6e6e73;
      --tertiary: #8e8e93;
      --separator: rgba(60, 60, 67, 0.15);
      --separator-strong: rgba(60, 60, 67, 0.24);
      --blue: #0071e3;
      --blue-deep: #0060c4;
      --green: #248a3d;
      --yellow: #a96e00;
      --orange: #c45f00;
      --red: #c4262e;
      --shadow: 0 24px 70px rgba(34, 91, 148, 0.14), 0 2px 10px rgba(25, 58, 92, 0.06);
      --card-shadow: 0 16px 40px rgba(34, 91, 148, 0.1), 0 1px 2px rgba(25, 58, 92, 0.04);
      --mono: "SF Mono", "SFMono-Regular", ui-monospace, Menlo, Monaco, Consolas, monospace;
      --sans: -apple-system, BlinkMacSystemFont, "SF Pro Display", "SF Pro Text", "Helvetica Neue", Arial, sans-serif;
    }

    * {
      box-sizing: border-box;
    }

    html {
      min-height: 100%;
      background: var(--page);
    }

    body {
      margin: 0;
      min-height: 100dvh;
      color: var(--text);
      font-family: var(--sans);
      letter-spacing: 0;
      background:
        linear-gradient(145deg, #e7f3ff 0%, #f7fbff 44%, #eef7ff 100%),
        var(--page);
      -webkit-font-smoothing: antialiased;
      text-rendering: optimizeLegibility;
    }

    button,
    input {
      font: inherit;
    }

    button {
      border: 0;
      cursor: pointer;
      user-select: none;
      transition:
        transform 180ms cubic-bezier(0.2, 0.8, 0.2, 1),
        background 180ms ease,
        color 180ms ease,
        border-color 180ms ease,
        opacity 180ms ease;
    }

    button:active {
      transform: scale(0.985);
    }

    button:disabled {
      cursor: not-allowed;
      opacity: 0.45;
    }

    .hidden {
      display: none !important;
    }

    .login {
      min-height: 100dvh;
      display: grid;
      place-items: center;
      padding: 24px;
    }

    .login-card {
      width: min(100%, 430px);
      display: grid;
      gap: 22px;
      padding: 28px;
      border: 1px solid rgba(0, 113, 227, 0.16);
      border-radius: 30px;
      background:
        linear-gradient(180deg, rgba(245, 250, 255, 0.96), rgba(255, 255, 255, 0.82)),
        rgba(255, 255, 255, 0.86);
      backdrop-filter: blur(30px) saturate(1.6);
      -webkit-backdrop-filter: blur(30px) saturate(1.6);
      box-shadow: var(--shadow), inset 0 1px 0 rgba(255, 255, 255, 0.9);
    }

    .app-icon {
      width: 54px;
      height: 54px;
      display: grid;
      place-items: center;
      border-radius: 16px;
      color: white;
      background:
        linear-gradient(180deg, rgba(255, 255, 255, 0.18), rgba(255, 255, 255, 0)),
        linear-gradient(145deg, #0071e3, #4aa3ff);
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.28), 0 12px 28px rgba(0, 113, 227, 0.26);
    }

    .app-icon svg {
      width: 28px;
      height: 28px;
      stroke-width: 1.7;
    }

    .login-copy {
      display: grid;
      gap: 7px;
    }

    .eyebrow {
      margin: 0;
      color: var(--secondary);
      font-size: 0.78rem;
      font-weight: 700;
      letter-spacing: 0;
    }

    h1,
    h2,
    p {
      margin: 0;
    }

    .login-card h1 {
      font-size: 2rem;
      line-height: 1.06;
      font-weight: 760;
      letter-spacing: 0;
    }

    .login-card p:not(.eyebrow):not(.login-error) {
      color: var(--secondary);
      line-height: 1.45;
      font-size: 0.95rem;
    }

    .form-block {
      display: grid;
      gap: 9px;
    }

    label {
      color: var(--secondary);
      font-size: 0.84rem;
      font-weight: 650;
    }

    input {
      width: 100%;
      height: 48px;
      border: 1px solid var(--separator);
      border-radius: 15px;
      outline: none;
      padding: 0 14px;
      background: rgba(255, 255, 255, 0.82);
      color: var(--text);
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.9);
    }

    input:focus {
      border-color: rgba(0, 113, 227, 0.58);
      box-shadow: 0 0 0 4px rgba(0, 113, 227, 0.12);
    }

    .login-error {
      min-height: 20px;
      color: var(--red);
      font-size: 0.84rem;
      font-weight: 650;
    }

    .app-shell {
      width: min(1320px, calc(100% - 32px));
      margin: 0 auto;
      padding: 24px 0;
    }

    .window {
      min-height: calc(100dvh - 48px);
      display: grid;
      grid-template-columns: 306px minmax(0, 1fr);
      overflow: hidden;
      border: 1px solid rgba(0, 113, 227, 0.16);
      border-radius: 34px;
      background:
        linear-gradient(135deg, rgba(232, 244, 255, 0.78), rgba(250, 253, 255, 0.9) 42%, rgba(238, 247, 255, 0.82)),
        rgba(245, 250, 255, 0.86);
      backdrop-filter: blur(34px) saturate(1.45);
      -webkit-backdrop-filter: blur(34px) saturate(1.45);
      box-shadow: var(--shadow), inset 0 1px 0 rgba(255, 255, 255, 0.9);
    }

    .sidebar {
      display: flex;
      flex-direction: column;
      gap: 20px;
      padding: 24px;
      border-right: 1px solid rgba(0, 113, 227, 0.12);
      background:
        linear-gradient(180deg, rgba(233, 245, 255, 0.84), rgba(246, 251, 255, 0.72)),
        rgba(245, 250, 255, 0.78);
    }

    .brand {
      display: flex;
      align-items: center;
      gap: 13px;
      min-width: 0;
    }

    .brand-title {
      min-width: 0;
    }

    .brand-title strong {
      display: block;
      font-size: 1rem;
      font-weight: 760;
      white-space: nowrap;
    }

    .brand-title span {
      display: block;
      margin-top: 3px;
      color: var(--secondary);
      font-size: 0.8rem;
      font-weight: 560;
      white-space: nowrap;
    }

    .sidebar-card {
      border: 1px solid var(--blue-border);
      border-radius: 24px;
      background:
        linear-gradient(180deg, rgba(245, 250, 255, 0.96), rgba(255, 255, 255, 0.78)),
        rgba(255, 255, 255, 0.86);
      box-shadow: var(--card-shadow), inset 0 1px 0 rgba(255, 255, 255, 0.94);
      overflow: hidden;
    }

    .status-grid {
      display: grid;
    }

    .metric {
      display: grid;
      gap: 4px;
      padding: 13px 14px;
      border-top: 1px solid var(--separator);
    }

    .metric:first-child {
      border-top: 0;
    }

    .metric span {
      color: var(--secondary);
      font-size: 0.74rem;
      font-weight: 650;
    }

    .metric strong {
      min-width: 0;
      overflow: hidden;
      color: var(--text);
      font-family: var(--mono);
      font-size: 0.78rem;
      font-weight: 650;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .sidebar-actions {
      display: grid;
      gap: 10px;
      margin-top: auto;
    }

    .content {
      min-width: 0;
      padding: 22px;
      display: grid;
      align-content: start;
      gap: 14px;
      background:
        linear-gradient(180deg, rgba(255, 255, 255, 0.24), rgba(232, 244, 255, 0.28)),
        transparent;
    }

    .content-head {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 14px;
      padding: 0 4px;
    }

    .title-block {
      display: grid;
      gap: 3px;
      min-width: 0;
    }

    .content-head h1 {
      font-size: 1.32rem;
      line-height: 1.12;
      font-weight: 760;
      letter-spacing: 0;
    }

    .content-head p {
      color: var(--secondary);
      font-size: 0.82rem;
      line-height: 1.35;
    }

    .content-head .eyebrow {
      font-size: 0.72rem;
      font-weight: 680;
    }

    .headline-actions {
      display: flex;
      align-items: center;
      gap: 10px;
      flex: 0 0 auto;
    }

    .button {
      min-height: 40px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
      padding: 0 16px;
      border: 1px solid rgba(0, 113, 227, 0.14);
      border-radius: 999px;
      background: rgba(247, 251, 255, 0.84);
      color: var(--text);
      font-weight: 700;
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.84);
    }

    .button:hover {
      background: rgba(255, 255, 255, 0.98);
    }

    .button.primary {
      border-color: rgba(0, 113, 227, 0.2);
      background: var(--blue);
      color: white;
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.22);
    }

    .button.primary:hover {
      background: var(--blue-deep);
    }

    .button svg {
      width: 16px;
      height: 16px;
      stroke-width: 1.85;
    }

    .summary {
      display: grid;
      grid-template-columns: minmax(0, 1.6fr) minmax(240px, 0.8fr);
      gap: 16px;
    }

    .summary-card {
      position: relative;
      overflow: hidden;
      display: grid;
      gap: 14px;
      padding: 20px;
      min-height: 146px;
      border: 1px solid rgba(0, 113, 227, 0.16);
      border-radius: 28px;
      background:
        linear-gradient(180deg, rgba(245, 250, 255, 0.96), rgba(255, 255, 255, 0.82)),
        rgba(255, 255, 255, 0.86);
      box-shadow: var(--card-shadow), inset 0 1px 0 rgba(255, 255, 255, 0.94);
    }

    .summary-kicker {
      color: var(--secondary);
      font-size: 0.78rem;
      font-weight: 700;
    }

    .summary-email {
      min-width: 0;
      overflow: hidden;
      font-size: 1.35rem;
      line-height: 1.15;
      font-weight: 760;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .summary-subtitle {
      color: var(--secondary);
      font-family: var(--mono);
      font-size: 0.78rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .summary-metrics {
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 10px;
    }

    .mini-metric {
      display: grid;
      gap: 5px;
      padding: 14px;
      border: 1px solid rgba(0, 113, 227, 0.13);
      border-radius: 22px;
      background:
        linear-gradient(180deg, rgba(245, 250, 255, 0.92), rgba(255, 255, 255, 0.78)),
        rgba(255, 255, 255, 0.84);
    }

    .mini-metric span {
      color: var(--secondary);
      font-size: 0.76rem;
      font-weight: 650;
    }

    .mini-metric strong {
      color: var(--text);
      font-family: var(--mono);
      font-size: 1rem;
      font-weight: 760;
    }

    .section-card {
      overflow: hidden;
      border: 1px solid rgba(0, 113, 227, 0.16);
      border-radius: 30px;
      background:
        linear-gradient(180deg, rgba(245, 250, 255, 0.94), rgba(255, 255, 255, 0.78)),
        rgba(255, 255, 255, 0.84);
      box-shadow: var(--card-shadow), inset 0 1px 0 rgba(255, 255, 255, 0.9);
    }

    .section-head {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 16px;
      padding: 18px 20px;
      border-bottom: 1px solid rgba(0, 113, 227, 0.12);
      background: rgba(232, 244, 255, 0.34);
    }

    .section-head h2 {
      font-size: 1.05rem;
      font-weight: 760;
    }

    .section-head p {
      color: var(--secondary);
      font-size: 0.85rem;
      font-weight: 600;
      white-space: nowrap;
    }

    .list {
      min-height: 220px;
    }

    .cards {
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 14px;
      padding: 16px;
    }

    .account-card {
      display: grid;
      gap: 17px;
      min-width: 0;
      padding: 18px;
      border: 1px solid var(--separator);
      border-radius: 26px;
      background: rgba(255, 255, 255, 0.82);
      box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.92);
      animation: rise 420ms cubic-bezier(0.2, 0.8, 0.2, 1) both;
      animation-delay: calc(var(--i) * 38ms);
    }

    .account-card.current {
      border-color: rgba(0, 113, 227, 0.36);
      background:
        linear-gradient(180deg, rgba(245, 250, 255, 0.96), rgba(255, 255, 255, 0.82)),
        rgba(255, 255, 255, 0.86);
      box-shadow: inset 0 0 0 1px rgba(0, 113, 227, 0.08), var(--card-shadow);
    }

    @keyframes rise {
      from {
        opacity: 0;
        transform: translateY(8px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }

    .card-head {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      gap: 12px;
      min-width: 0;
    }

    .identity {
      min-width: 0;
      display: grid;
      gap: 5px;
    }

    .email {
      display: block;
      overflow: hidden;
      color: var(--text);
      font-size: 1.03rem;
      font-weight: 760;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .id {
      display: block;
      overflow: hidden;
      color: var(--secondary);
      font-family: var(--mono);
      font-size: 0.72rem;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .card-badges {
      display: flex;
      justify-content: flex-end;
      gap: 7px;
      flex-wrap: wrap;
      flex: 0 0 auto;
      max-width: 48%;
    }

    .pill {
      min-height: 26px;
      display: inline-flex;
      align-items: center;
      padding: 0 10px;
      border: 1px solid var(--separator);
      border-radius: 999px;
      background: rgba(255, 255, 255, 0.74);
      color: var(--secondary);
      font-size: 0.75rem;
      font-weight: 700;
      white-space: nowrap;
    }

    .pill.current {
      border-color: rgba(0, 113, 227, 0.24);
      background: rgba(0, 113, 227, 0.1);
      color: var(--blue);
    }

    .pill.warn {
      border-color: rgba(169, 110, 0, 0.24);
      background: rgba(169, 110, 0, 0.1);
      color: var(--yellow);
    }

    .quota-section {
      display: grid;
      gap: 12px;
      padding: 14px;
      border: 1px solid var(--separator);
      border-radius: 22px;
      background: rgba(247, 247, 249, 0.74);
    }

    .quota-item {
      display: grid;
      gap: 8px;
    }

    .quota-header {
      display: flex;
      align-items: center;
      gap: 8px;
      color: var(--secondary);
      font-size: 0.8rem;
    }

    .quota-header svg {
      width: 15px;
      height: 15px;
      flex: 0 0 auto;
      stroke-width: 1.75;
    }

    .quota-label {
      flex: 1;
      font-weight: 700;
    }

    .quota-pct {
      font-family: var(--mono);
      font-size: 0.84rem;
      font-weight: 780;
    }

    .quota-pct.high { color: var(--green); }
    .quota-pct.medium { color: var(--yellow); }
    .quota-pct.low { color: var(--orange); }
    .quota-pct.critical { color: var(--red); }

    .quota-bar-track {
      height: 7px;
      overflow: hidden;
      border-radius: 999px;
      background: rgba(60, 60, 67, 0.13);
    }

    .quota-bar {
      height: 100%;
      border-radius: 999px;
      transition: width 300ms ease;
    }

    .quota-bar.high { background: var(--green); }
    .quota-bar.medium { background: var(--yellow); }
    .quota-bar.low { background: var(--orange); }
    .quota-bar.critical { background: var(--red); }

    .quota-reset {
      overflow: hidden;
      color: var(--tertiary);
      font-family: var(--mono);
      font-size: 0.72rem;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .quota-empty {
      padding: 13px;
      border-radius: 16px;
      background: rgba(255, 255, 255, 0.62);
      color: var(--secondary);
      text-align: center;
      font-size: 0.83rem;
      font-weight: 650;
    }

    .quota-error {
      overflow: hidden;
      padding: 10px 12px;
      border-radius: 16px;
      background: rgba(196, 38, 46, 0.08);
      color: var(--red);
      font-size: 0.78rem;
      font-weight: 700;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .card-meta-grid {
      display: grid;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 10px;
    }

    .meta-cell {
      min-width: 0;
      display: grid;
      gap: 5px;
    }

    .label {
      color: var(--secondary);
      font-size: 0.72rem;
      font-weight: 680;
    }

    .value {
      overflow: hidden;
      color: var(--text);
      font-family: var(--mono);
      font-size: 0.78rem;
      font-weight: 650;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .account-card > .actions {
      display: flex;
      justify-content: flex-end;
    }

    .empty,
    .error-box {
      padding: 56px 24px;
      color: var(--secondary);
      text-align: center;
      font-weight: 650;
    }

    .error-box {
      color: var(--red);
    }

    .skeleton {
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 14px;
      padding: 16px;
    }

    .skeleton-line {
      height: 230px;
      border-radius: 26px;
      background: linear-gradient(90deg, rgba(232, 244, 255, 0.52), rgba(255, 255, 255, 0.92), rgba(232, 244, 255, 0.52));
      background-size: 220% 100%;
      animation: shimmer 1.5s infinite;
    }

    @keyframes shimmer {
      from { background-position: 120% 0; }
      to { background-position: -120% 0; }
    }

    .toast {
      position: fixed;
      right: 20px;
      bottom: 20px;
      max-width: min(430px, calc(100% - 40px));
      padding: 13px 16px;
      border: 1px solid rgba(0, 113, 227, 0.16);
      border-radius: 18px;
      background:
        linear-gradient(180deg, rgba(245, 250, 255, 0.96), rgba(255, 255, 255, 0.84)),
        rgba(255, 255, 255, 0.9);
      backdrop-filter: blur(24px) saturate(1.45);
      -webkit-backdrop-filter: blur(24px) saturate(1.45);
      box-shadow: var(--shadow);
      color: var(--text);
      font-weight: 700;
      opacity: 0;
      pointer-events: none;
      transform: translateY(16px);
      transition: opacity 220ms ease, transform 220ms cubic-bezier(0.2, 0.8, 0.2, 1);
    }

    .toast.show {
      opacity: 1;
      transform: translateY(0);
    }

    @media (max-width: 980px) {
      .app-shell {
        width: min(100% - 22px, 760px);
        padding: 12px 0;
      }

      .window {
        min-height: calc(100dvh - 24px);
        grid-template-columns: 1fr;
        border-radius: 28px;
      }

      .sidebar {
        border-right: 0;
        border-bottom: 1px solid var(--separator);
      }

      .sidebar-actions {
        grid-template-columns: 1fr 1fr;
      }

      .content {
        padding: 20px 14px 14px;
      }

      .content-head {
        flex-direction: column;
      }

      .headline-actions {
        width: 100%;
      }

      .headline-actions .button {
        flex: 1;
      }

      .summary {
        grid-template-columns: 1fr;
      }

      .cards,
      .skeleton {
        grid-template-columns: 1fr;
      }
    }

    @media (max-width: 620px) {
      .app-shell {
        width: 100%;
        padding: 0;
      }

      .window {
        min-height: 100dvh;
        border-radius: 0;
        border-left: 0;
        border-right: 0;
      }

      .sidebar {
        padding: 18px 16px;
      }

      .content-head h1 {
        font-size: 1.25rem;
      }

      .summary-metrics,
      .card-meta-grid {
        grid-template-columns: 1fr;
      }

      .card-head {
        display: grid;
      }

      .card-badges {
        max-width: none;
        justify-content: flex-start;
      }

      .account-card > .actions .button {
        width: 100%;
      }
    }
  </style>
</head>
<body>
  <main id="loginView" class="login hidden">
    <form class="login-card" id="loginForm">
      <div class="app-icon" aria-hidden="true">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <path d="M6 8.2 12 4l6 4.2v7.6L12 20l-6-4.2V8.2Z"/>
          <path d="M9 10.2h6M9 13.8h6"/>
        </svg>
      </div>
      <div class="login-copy">
        <p class="eyebrow">局域网私有桥接</p>
        <h1>Codex 账号控制台</h1>
        <p>从 CT 保存的账号中切换 Codex 登录态，并重启 Codex App。</p>
      </div>
      <div class="form-block">
        <label for="password">访问密码</label>
        <input id="password" name="password" type="password" autocomplete="current-password" required>
      </div>
      <p id="loginError" class="login-error"></p>
      <button class="button primary" type="submit">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M10 17l5-5-5-5"/><path d="M15 12H3"/><path d="M15 5h3a3 3 0 0 1 3 3v8a3 3 0 0 1-3 3h-3"/></svg>
        进入面板
      </button>
    </form>
  </main>

  <main id="appView" class="app-shell hidden">
    <div class="window">
      <aside class="sidebar">
        <div class="brand">
          <div class="app-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <path d="M6 8.2 12 4l6 4.2v7.6L12 20l-6-4.2V8.2Z"/>
              <path d="M9 10.2h6M9 13.8h6"/>
            </svg>
          </div>
          <div class="brand-title">
            <strong>Codex Bridge</strong>
            <span>CT 账号投影</span>
          </div>
        </div>

        <section class="sidebar-card">
          <div class="status-grid" id="statusGrid">
            <div class="metric"><span>Codex</span><strong>加载中</strong></div>
            <div class="metric"><span>钥匙串</span><strong>加载中</strong></div>
            <div class="metric"><span>当前投影</span><strong>加载中</strong></div>
            <div class="metric"><span>Codex 目录</span><strong>加载中</strong></div>
          </div>
        </section>

        <div class="sidebar-actions">
          <button id="refreshButton" class="button" type="button">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M20 12a8 8 0 1 1-2.34-5.66"/><path d="M20 4v5h-5"/></svg>
            刷新
          </button>
          <button id="logoutButton" class="button" type="button">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><path d="M16 17l5-5-5-5"/><path d="M21 12H9"/></svg>
            退出
          </button>
        </div>
      </aside>

      <section class="content">
        <header class="content-head">
          <div class="title-block">
            <p class="eyebrow">Codex 账号</p>
            <h1>账号切换</h1>
            <p>只读取 CT 已保存的数据，不刷新 token，不触发账号检测。</p>
          </div>
          <div class="headline-actions">
            <button class="button" type="button" onclick="loadAll()">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M20 12a8 8 0 1 1-2.34-5.66"/><path d="M20 4v5h-5"/></svg>
              更新
            </button>
          </div>
        </header>

        <section class="section-card">
          <div class="section-head">
            <div>
              <h2>已保存账号</h2>
              <p id="accountCount">加载中</p>
            </div>
          </div>
          <div id="accountList" class="list">
            <div class="skeleton">
              <div class="skeleton-line"></div>
              <div class="skeleton-line"></div>
            </div>
          </div>
        </section>
      </section>
    </div>
  </main>

  <div id="toast" class="toast"></div>

  <script>
    const loginView = document.getElementById('loginView');
    const appView = document.getElementById('appView');
    const accountList = document.getElementById('accountList');
    const accountCount = document.getElementById('accountCount');
    const statusGrid = document.getElementById('statusGrid');
    const toast = document.getElementById('toast');
    let accounts = [];
    let busyAccountId = null;

    const iconSwitch = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M7 7h11l-3-3"/><path d="M17 17H6l3 3"/><path d="M18 7l-3 3"/><path d="M6 17l3-3"/></svg>';
    const iconGauge = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M4 14a8 8 0 1 1 16 0"/><path d="M12 14l4-4"/><path d="M7 18h10"/></svg>';

    init();

    async function init() {
      const session = await request('/api/session/status', { allowError: true });
      if (session && session.authenticated) {
        showApp();
        await loadAll();
      } else {
        showLogin();
      }
    }

    document.getElementById('loginForm').addEventListener('submit', async (event) => {
      event.preventDefault();
      const password = document.getElementById('password').value;
      const loginError = document.getElementById('loginError');
      loginError.textContent = '';
      const result = await request('/api/session/login', {
        method: 'POST',
        body: { password },
        allowError: true
      });
      if (result && result.ok) {
        showApp();
        await loadAll();
      } else {
        loginError.textContent = result && result.error ? result.error : '登录失败，请检查访问密码';
      }
    });

    document.getElementById('refreshButton').addEventListener('click', loadAll);
    document.getElementById('logoutButton').addEventListener('click', async () => {
      await request('/api/session/logout', { method: 'POST', body: {}, allowError: true });
      showLogin();
    });

    function showLogin() {
      loginView.classList.remove('hidden');
      appView.classList.add('hidden');
      document.getElementById('password').focus();
    }

    function showApp() {
      loginView.classList.add('hidden');
      appView.classList.remove('hidden');
    }

    async function loadAll() {
      renderLoading();
      const [accountData, statusData] = await Promise.all([
        request('/api/codex/accounts', { allowError: true }),
        request('/api/codex/status', { allowError: true })
      ]);

      if (accountData && accountData.error === 'unauthorized') {
        showLogin();
        return;
      }

      if (statusData && !statusData.error) renderStatus(statusData);
      if (accountData && !accountData.error) {
        accounts = accountData.accounts || [];
        renderAccounts();
      } else {
        renderError(accountData && accountData.error ? accountData.error : '无法加载账号列表');
      }
    }

    function renderLoading() {
      accountCount.textContent = '加载中';
      accountList.innerHTML = '<div class="skeleton"><div class="skeleton-line"></div><div class="skeleton-line"></div></div>';
    }

    function renderStatus(status) {
      const projection = status.currentProjection ? status.currentProjection.email : '无';
      statusGrid.innerHTML = [
        metric('Codex', status.codexRunning ? '运行中' : '未运行'),
        metric('钥匙串', status.keychainEntryPresent ? '已写入' : '未写入'),
        metric('当前投影', projection),
        metric('Codex 目录', status.codexHome)
      ].join('');
    }

    function metric(label, value) {
      return `<div class="metric"><span>${escapeHtml(label)}</span><strong title="${escapeHtml(value)}">${escapeHtml(value)}</strong></div>`;
    }

    function renderAccounts() {
      accountCount.textContent = `${accounts.length} 个账号`;

      if (!accounts.length) {
        accountList.innerHTML = '<div class="empty">没有在 CT 中找到 Codex 账号。</div>';
        return;
      }

      const cards = accounts.map((account, index) => {
        const status = account.isCurrent
          ? '<span class="pill current">当前</span>'
          : account.requiresReauth
            ? '<span class="pill warn">CT 标记需重登</span>'
            : '<span class="pill">可切换</span>';
        const material = account.authMode === 'apikey'
          ? (account.hasApiKey ? 'API Key 已保存' : '缺少 API Key')
          : (account.hasOAuthSnapshot ? 'OAuth 登录态' : '缺少登录态');
        const plan = account.planType || '未知';
        const lastUsed = formatTime(account.lastUsed);
        const usageUpdated = formatTime(account.usageUpdatedAt);
        const disabled = !account.canSwitch || account.isCurrent || busyAccountId;
        const label = busyAccountId === account.id
          ? '切换中'
          : account.isCurrent
            ? '当前账号'
            : account.canSwitch ? '切换' : '缺少凭据';
        return `
          <article class="account-card ${account.isCurrent ? 'current' : ''}" style="--i:${index}">
            <div class="card-head">
              <div class="identity">
                <span class="email" title="${escapeHtml(account.email)}">${escapeHtml(account.email)}</span>
                <span class="id" title="${escapeHtml(account.id)}">${escapeHtml(account.id)}</span>
              </div>
              <div class="card-badges">
                ${status}
                <span class="pill">${escapeHtml(plan)}</span>
              </div>
            </div>
            ${renderQuota(account)}
            <div class="card-meta-grid">
              <div class="meta-cell"><span class="label">登录态</span><div class="value">${escapeHtml(material)}</div></div>
              <div class="meta-cell"><span class="label">上次使用</span><div class="value">${escapeHtml(lastUsed)}</div></div>
              <div class="meta-cell"><span class="label">额度更新</span><div class="value">${escapeHtml(usageUpdated)}</div></div>
            </div>
            <div class="actions">
              <button class="button primary" data-switch="${escapeHtml(account.id)}" ${disabled ? 'disabled' : ''}>${iconSwitch}${label}</button>
            </div>
          </article>
        `;
      }).join('');

      accountList.innerHTML = `<div class="cards">${cards}</div>`;

      accountList.querySelectorAll('[data-switch]').forEach((button) => {
        button.addEventListener('click', () => switchAccount(button.getAttribute('data-switch')));
      });
    }

    async function switchAccount(accountId) {
      const account = accounts.find((item) => item.id === accountId);
      if (!account) return;
      busyAccountId = accountId;
      renderAccounts();
      const result = await request('/api/codex/switch', {
        method: 'POST',
        body: { accountId, restart: true },
        allowError: true
      });
      busyAccountId = null;
      if (result && !result.error) {
        showToast(`已切换到 ${result.account.email}`);
        await loadAll();
      } else {
        renderAccounts();
        showToast(result && result.error ? result.error : '切换失败');
      }
    }

    function renderQuota(account) {
      const parts = [];
      if (account.quota) {
        const quota = account.quota;
        const effective = getEffectiveQuotaPercentages(quota);
        if (isQuotaWindowPresent(quota, 'hourly')) {
          parts.push(renderQuotaItem({
            label: '5 小时额度',
            percentage: effective.hourly,
            resetTime: quota.hourly_reset_time
          }));
        }
        if (isQuotaWindowPresent(quota, 'weekly')) {
          parts.push(renderQuotaItem({
            label: '周额度',
            percentage: effective.weekly,
            resetTime: quota.weekly_reset_time
          }));
        }
      }

      if (!parts.length) {
        parts.push('<div class="quota-empty">暂无 CT 保存的额度信息</div>');
      }

      if (account.quotaError && account.quotaError.message) {
        parts.push(`<div class="quota-error" title="${escapeHtml(account.quotaError.message)}">${escapeHtml(account.quotaError.message)}</div>`);
      }

      return `<div class="quota-section">${parts.join('')}</div>`;
    }

    function renderQuotaItem(item) {
      const percentage = clampQuotaPercentage(item.percentage);
      const klass = quotaClass(percentage);
      return `
        <div class="quota-item">
          <div class="quota-header">
            ${iconGauge}
            <span class="quota-label">${escapeHtml(item.label)}</span>
            <span class="quota-pct ${klass}">${percentage}%</span>
          </div>
          <div class="quota-bar-track" aria-hidden="true">
            <div class="quota-bar ${klass}" style="width:${percentage}%"></div>
          </div>
          <div class="quota-reset">${escapeHtml(formatResetTime(item.resetTime))}</div>
        </div>
      `;
    }

    function getEffectiveQuotaPercentages(quota) {
      const hourly = clampQuotaPercentage(quota.hourly_percentage);
      const weekly = clampQuotaPercentage(quota.weekly_percentage);
      const weeklyBlocksHourly = isQuotaWindowPresent(quota, 'weekly') && weekly === 0;
      return {
        hourly: weeklyBlocksHourly && isQuotaWindowPresent(quota, 'hourly') ? 0 : hourly,
        weekly
      };
    }

    function isQuotaWindowPresent(quota, type) {
      const key = type === 'hourly' ? 'hourly_window_present' : 'weekly_window_present';
      return quota[key] !== false;
    }

    function quotaClass(percentage) {
      if (percentage >= 80) return 'high';
      if (percentage >= 40) return 'medium';
      if (percentage >= 10) return 'low';
      return 'critical';
    }

    function clampQuotaPercentage(value) {
      const number = Number(value);
      if (!Number.isFinite(number)) return 0;
      return Math.max(0, Math.min(100, Math.round(number)));
    }

    function formatResetTime(value) {
      if (!value) return '重置时间未知';
      const resetAt = new Date(value * 1000);
      if (Number.isNaN(resetAt.getTime())) return '重置时间未知';
      const diffMs = resetAt.getTime() - Date.now();
      if (diffMs <= 0) return '已重置';
      return `重置：${formatDuration(diffMs)} (${formatAbsoluteTime(resetAt)})`;
    }

    function formatDuration(ms) {
      const totalMinutes = Math.max(1, Math.ceil(ms / 60000));
      const days = Math.floor(totalMinutes / 1440);
      const hours = Math.floor((totalMinutes % 1440) / 60);
      const minutes = totalMinutes % 60;
      const chunks = [];
      if (days) chunks.push(`${days}天`);
      if (hours) chunks.push(`${hours}小时`);
      if (!days && minutes) chunks.push(`${minutes}分钟`);
      return chunks.slice(0, 2).join(' ');
    }

    function formatAbsoluteTime(date) {
      return new Intl.DateTimeFormat('zh-CN', {
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        hour12: false
      }).format(date);
    }

    function formatTime(value) {
      if (!value) return '从未';
      const date = new Date(value * 1000);
      if (Number.isNaN(date.getTime())) return '未知';
      return new Intl.DateTimeFormat('zh-CN', {
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        hour12: false
      }).format(date);
    }

    async function request(url, options = {}) {
      const init = { method: options.method || 'GET', headers: {} };
      if (options.body !== undefined) {
        init.headers['Content-Type'] = 'application/json';
        init.body = JSON.stringify(options.body);
      }
      try {
        const response = await fetch(url, init);
        const data = await response.json().catch(() => ({}));
        if (!response.ok && !options.allowError) throw new Error(data.error || response.statusText);
        if (!response.ok) return { error: data.error || response.statusText };
        return data;
      } catch (error) {
        if (options.allowError) return { error: error.message };
        throw error;
      }
    }

    function showToast(message) {
      toast.textContent = message;
      toast.classList.add('show');
      clearTimeout(showToast.timer);
      showToast.timer = setTimeout(() => toast.classList.remove('show'), 3200);
    }

    function renderError(message) {
      accountList.innerHTML = `<div class="error-box">${escapeHtml(message)}</div>`;
    }

    function escapeHtml(value) {
      return String(value == null ? '' : value)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
    }
  </script>
</body>
</html>
"##;
