const agents = document.querySelector('#agents');
const activity = document.querySelector('#activity');
const notice = document.querySelector('#notice');
const refresh = document.querySelector('#refresh');
const session = sessionStorage.getItem('garden-session');
const MiB = 1024 * 1024;
const MODES = {
  manual_review: 'Manual review — every attachment waits for you',
  trusted_companion: 'Trusted companion — routine attachments you ask for commit automatically',
  autonomous_gardening: 'Autonomous gardening — also self-started backfill, within limits',
};

async function request(payload) {
  if (!session) throw new Error('Open Agent trust from your live Garden launch link.');
  const headers = { Authorization: 'Bearer ' + session };
  if (payload) headers['Content-Type'] = 'application/json';
  const response = await fetch('/api/trust', {
    method: payload ? 'POST' : 'GET',
    headers,
    body: payload ? JSON.stringify(payload) : undefined,
    cache: 'no-store',
  });
  if (response.status === 401) throw new Error('Reopen the live Garden to refresh your owner session.');
  const value = await response.json().catch(() => ({}));
  if (!response.ok) {
    const error = new Error(value.message || 'Agent trust is unavailable.');
    error.code = value.code;
    throw error;
  }
  return value;
}

function element(tag, text, className) {
  const node = document.createElement(tag);
  if (text !== undefined) node.textContent = text;
  if (className) node.className = className;
  return node;
}

function renderAgent(agent, limits) {
  const section = element('section');
  section.append(element('h2', agent.clientId));
  const automatic = agent.effective.routineAttachments === 'automatic';
  section.append(element('p', automatic ? 'Routine attachments commit automatically.' : 'Every attachment waits for your review.',
    automatic ? 'mode-auto' : 'mode-manual'));
  const fields = element('dl');
  const field = (label, value) => fields.append(element('dt', label), element('dd', value));
  field('Granted scopes', agent.scopes.join(', ') || 'none');
  field('Record visibility', String(agent.visibility ?? 'unknown'));
  field('Policy revision', String(agent.policy.revision) + (agent.policy.updatedAt ? ' · ' + new Date(agent.policy.updatedAt).toLocaleString() : ' · default'));
  section.append(fields);
  if (!agent.eligibleForTrust) {
    section.append(element('p', 'This credential lacks read and media:attach, so it cannot be trusted with attachments. Grants are changed with the owner CLI.', 'muted'));
    return section;
  }
  const form = element('div', undefined, 'actions');
  const modeLabel = element('label', 'Trust mode');
  const mode = element('select');
  for (const [value, text] of Object.entries(MODES)) {
    const option = element('option', text);
    option.value = value;
    option.selected = value === agent.policy.mode;
    mode.append(option);
  }
  modeLabel.append(mode);
  const countLabel = element('label', 'Automatic commits per day');
  const count = element('input');
  Object.assign(count, { type: 'number', min: 1, max: limits.maxAutoCommitsPerDay, value: agent.policy.maxAutoCommitsPerDay });
  countLabel.append(count);
  const bytesLabel = element('label', 'Automatic MiB per day');
  const bytes = element('input');
  Object.assign(bytes, { type: 'number', min: 1, max: limits.maxAutoBytesPerDay / MiB, value: Math.round(agent.policy.maxAutoBytesPerDay / MiB) });
  bytesLabel.append(bytes);
  const save = element('button', 'Save trust', 'save');
  save.type = 'button';
  const revoke = element('button', 'Return to manual review', 'revoke');
  revoke.type = 'button';
  revoke.disabled = agent.policy.mode === 'manual_review';
  const apply = async (chosen) => {
    save.disabled = revoke.disabled = true;
    notice.textContent = 'Saving trust for ' + agent.clientId + '…';
    try {
      await request({
        action: 'set', clientId: agent.clientId, mode: chosen,
        maxAutoCommitsPerDay: Number(count.value), maxAutoBytesPerDay: Math.round(Number(bytes.value) * MiB),
        expectedRevision: agent.policy.revision,
      });
      notice.textContent = 'Saved. ' + agent.clientId + ' now uses ' + chosen.replaceAll('_', ' ') + '.';
      await load();
    } catch (error) {
      notice.textContent = error.message;
      save.disabled = false;
      revoke.disabled = agent.policy.mode === 'manual_review';
    }
  };
  save.addEventListener('click', () => apply(mode.value));
  revoke.addEventListener('click', () => apply('manual_review'));
  form.append(modeLabel, countLabel, bytesLabel, save, revoke);
  section.append(form);
  return section;
}

function mechanism(item) {
  const by = item.authorization && item.authorization.mechanism;
  if (by === 'owner_granted_trust_policy') return 'trust policy (rev ' + item.authorization.policy.policyRevision + ')';
  if (by === 'owner_manual_approval') return 'you approved';
  return '—';
}

function renderActivity(items) {
  activity.replaceChildren();
  if (!items.length) {
    activity.append(element('p', 'No attachment operations yet.', 'muted'));
    return;
  }
  const table = element('table');
  const head = element('tr');
  for (const title of ['When', 'Agent', 'Memory', 'File', 'Status', 'Authorized by', 'Digest', '']) head.append(element('th', title));
  table.append(head);
  for (const item of items) {
    const row = element('tr');
    const when = item.createdAt ? new Date(item.createdAt * 1000).toLocaleString() : '';
    const digest = item.verification?.sha256 || item.file?.sha256 || '';
    const status = item.reviewReason && item.status === 'pending_owner_review'
      ? 'pending review: ' + item.reviewReason.replaceAll('_', ' ') : String(item.status).replaceAll('_', ' ');
    for (const text of [when, item.proposedBy, item.memoryTitle, item.file?.fileName || '', status, mechanism(item), digest.slice(0, 12)]) {
      row.append(element('td', text));
    }
    const cell = element('td');
    if (item.status === 'approved' && item.reviewId) {
      const retract = element('button', 'Retract link', 'revoke');
      retract.type = 'button';
      retract.addEventListener('click', async () => {
        if (!confirm('Remove this media link from "' + item.memoryTitle + '"? The original file and receipt stay in the Garden.')) return;
        retract.disabled = true;
        try {
          await request({ action: 'retract', operationId: item.operationId, reviewId: item.reviewId });
          notice.textContent = 'Link retracted.';
          await load();
        } catch (error) {
          notice.textContent = error.message;
          retract.disabled = false;
        }
      });
      cell.append(retract);
    }
    row.append(cell);
    table.append(row);
  }
  activity.append(table);
}

async function load() {
  refresh.disabled = true;
  try {
    const overview = await request();
    agents.replaceChildren(...overview.agents.map(agent => renderAgent(agent, overview.limits)));
    renderActivity(overview.activity || []);
  } catch (error) {
    notice.textContent = error.message;
  } finally {
    refresh.disabled = false;
  }
}

refresh.addEventListener('click', load);
load();
