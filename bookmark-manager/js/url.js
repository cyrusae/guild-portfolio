// Known tracker/analytics query parameters to strip from URLs.
const TRACKER_PARAMS = [
  // UTM
  'utm_source', 'utm_medium', 'utm_campaign', 'utm_content', 'utm_term',
  'utm_id', 'utm_source_platform', 'utm_creative_format', 'utm_marketing_tactic',
  // Ad platform click IDs
  'fbclid', 'gclid', 'gclsrc', 'dclid', 'msclkid', 'twclid', 'igshid', 'ttclid',
  // Email/marketing platforms
  'mc_eid', 'mc_cid', '_hsenc', '_hsmi', 'hsCtaTracking', 'mkt_tok',
  // Misc
  'ref', 'referrer', 'source',
];

/**
 * Validates a raw URL string.
 * @param {string} raw
 * @returns {{ valid: true, url: string } | { valid: false, error: string }}
 */
export function validateUrl(raw) {
  const trimmed = raw.trim();

  if (!trimmed) {
    return { valid: false, error: 'URL is required.' };
  }

  if (!/^https?:\/\//i.test(trimmed)) {
    return { valid: false, error: 'URL must start with http:// or https://.' };
  }

  let parsed;
  try {
    parsed = new URL(trimmed);
  } catch {
    return { valid: false, error: 'URL is not valid.' };
  }

  // Require at least one dot in the hostname (rules out bare labels like "localhost")
  if (!parsed.hostname || !parsed.hostname.includes('.')) {
    return { valid: false, error: 'URL must include a valid domain (e.g. example.com).' };
  }

  return { valid: true, url: trimmed };
}

/**
 * Removes known tracker query parameters from a URL.
 * Returns the cleaned URL string.
 * @param {string} raw
 * @returns {string}
 */
export function cleanUrl(raw) {
  let parsed;
  try {
    parsed = new URL(raw.trim());
  } catch {
    return raw.trim();
  }

  for (const key of [...parsed.searchParams.keys()]) {
    if (TRACKER_PARAMS.includes(key) || key.startsWith('utm_')) {
      parsed.searchParams.delete(key);
    }
  }

  // Remove trailing '?' if all params were stripped
  return parsed.toString();
}
