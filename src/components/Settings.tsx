import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ProviderConfig {
  id: string;
  name: string;
  provider_type: string;
  account_type: string;
  api_key: string | null;
  enabled: boolean;
  budget_limit: number | null;
}

const ACCOUNT_TYPE_OPTIONS = [
  { value: "api", label: "API (Pay per token)" },
  { value: "pro", label: "Pro ($20/month)" },
  { value: "max100", label: "Max ($100/month)" },
  { value: "max200", label: "Max ($200/month)" },
];

function Settings() {
  const [providers, setProviders] = useState<ProviderConfig[]>([]);
  const [autostart, setAutostart] = useState(false);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    invoke<ProviderConfig[]>("get_providers").then(setProviders);
    invoke<boolean>("get_autostart_enabled").then(setAutostart).catch(() => {});
  }, []);

  const updateProvider = (id: string, updates: Partial<ProviderConfig>) => {
    setProviders((prev) =>
      prev.map((p) => (p.id === id ? { ...p, ...updates } : p))
    );
    setSaved(false);
  };

  const toggleAutostart = async (enabled: boolean) => {
    try {
      await invoke("set_autostart_enabled", { enabled });
      setAutostart(enabled);
    } catch (err) {
      console.error("Failed to set autostart:", err);
    }
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await invoke("save_providers", { providers });
      setSaved(true);
    } catch (err) {
      console.error("Failed to save:", err);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="settings">
      <h2>Provider Configuration</h2>
      <div className="provider-list">
        {providers.map((provider) => (
          <div key={provider.id} className="provider-item">
            <div className="provider-header">
              <label className="toggle">
                <input
                  type="checkbox"
                  checked={provider.enabled}
                  onChange={(e) =>
                    updateProvider(provider.id, { enabled: e.target.checked })
                  }
                />
                <span>{provider.name}</span>
              </label>
            </div>
            {provider.enabled && (
              <div className="provider-fields">
                <div className="field">
                  <label>Account Type</label>
                  <select
                    value={provider.account_type}
                    onChange={(e) =>
                      updateProvider(provider.id, {
                        account_type: e.target.value,
                      })
                    }
                  >
                    {ACCOUNT_TYPE_OPTIONS.map((opt) => (
                      <option key={opt.value} value={opt.value}>
                        {opt.label}
                      </option>
                    ))}
                  </select>
                </div>

                {provider.account_type === "api" && (
                  <div className="field">
                    <label>API Key</label>
                    <input
                      type="password"
                      value={provider.api_key || ""}
                      onChange={(e) =>
                        updateProvider(provider.id, {
                          api_key: e.target.value || null,
                        })
                      }
                      placeholder="Enter API key..."
                    />
                  </div>
                )}

                {provider.account_type === "api" ? (
                  <div className="field">
                    <label>Budget Limit ($)</label>
                    <input
                      type="number"
                      value={provider.budget_limit ?? ""}
                      onChange={(e) =>
                        updateProvider(provider.id, {
                          budget_limit: e.target.value
                            ? parseFloat(e.target.value)
                            : null,
                        })
                      }
                      placeholder="No limit"
                    />
                  </div>
                ) : (
                  <div className="field-hint">
                    Quota limits are set automatically based on your plan.
                  </div>
                )}
              </div>
            )}
          </div>
        ))}
      </div>

      <button className="save-btn" onClick={handleSave} disabled={saving}>
        {saving ? "Saving..." : saved ? "Saved" : "Save Configuration"}
      </button>

      <div className="settings-section">
        <h2>General</h2>
        <div className="provider-item">
          <label className="toggle">
            <input
              type="checkbox"
              checked={autostart}
              onChange={(e) => toggleAutostart(e.target.checked)}
            />
            <span>Launch at login</span>
          </label>
        </div>
      </div>
    </div>
  );
}

export default Settings;
