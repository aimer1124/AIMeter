import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ProviderConfig {
  id: string;
  name: string;
  provider_type: string;
  api_key: string | null;
  enabled: boolean;
  budget_limit: number | null;
}

function Settings() {
  const [providers, setProviders] = useState<ProviderConfig[]>([]);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    invoke<ProviderConfig[]>("get_providers").then(setProviders);
  }, []);

  const updateProvider = (id: string, updates: Partial<ProviderConfig>) => {
    setProviders((prev) =>
      prev.map((p) => (p.id === id ? { ...p, ...updates } : p))
    );
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await invoke("save_providers", { providers });
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
              </div>
            )}
          </div>
        ))}
      </div>
      <button className="save-btn" onClick={handleSave} disabled={saving}>
        {saving ? "Saving..." : "Save Configuration"}
      </button>
    </div>
  );
}

export default Settings;
