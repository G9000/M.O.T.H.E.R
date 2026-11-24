const items = document.querySelector('#items');
const notice = document.querySelector('#notice');
const refresh = document.querySelector('#refresh');
const session = sessionStorage.getItem('garden-session');

async function request(payload) {
  if (!session) throw new Error('Open Attachment review from your live Garden launch link.');
  const headers = { Authorization: 'Bearer ' + session };
  if (payload) headers['Content-Type'] = 'application/json';
  const response = await fetch('/api/attachments', {
    method: payload ? 'POST' : 'GET',
    headers,
    body: payload ? JSON.stringify(payload) : undefined,
    cache: 'no-store',
  });
  if (response.status === 401) throw new Error('Reopen the live Garden to refresh your owner session.');
  const value = await response.json().catch(() => ({}));
  if (!response.ok) {
    const error = new Error(value.message || 'Attachment review is unavailable.');
    error.code = value.code;
    throw error;
  }
  return value;
}

function element(tag, text) {
  const node = document.createElement(tag);
  node.textContent = text;
  return node;
}

function base64(bytes) {
  let binary = '';
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary);
}

async function upload(item, file, policy) {
  if (!file || file.size < 1 || file.size > policy.maxFileBytes) {
    throw new Error('Select an original file up to ' + policy.maxFileBytes / (1024 * 1024) + ' MiB.');
  }
  const sha256 = Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', await file.arrayBuffer())),
    byte => byte.toString(16).padStart(2, '0')).join('');
  const operationId = item.operationId;
  const selection = await request({ action: 'choose', operationId, file: { fileName: file.name, sizeBytes: file.size, sha256 } });
  if (selection.status !== 'source_bytes_unavailable') return selection;
  let offset = selection.receivedBytes;
  while (offset < file.size) {
    const data = await file.slice(offset, offset + policy.maxChunkBytes).arrayBuffer();
    const payload = { action: 'upload', operationId, offset, contentBase64: base64(new Uint8Array(data)) };
    let result;
    try {
      result = await request(payload);
    } catch (error) {
      if (error.code !== 'RATE_LIMITED') throw error;
      await new Promise(resolve => setTimeout(resolve, 1000));
      result = await request(payload);
    }
    if (result.status !== 'source_bytes_unavailable') return result;
    if (result.receivedBytes <= offset) throw new Error('The transfer stopped advancing. Select the same file to resume.');
    offset = result.receivedBytes;
    notice.textContent = 'Staged ' + offset.toLocaleString() + ' of ' + file.size.toLocaleString() + ' bytes.';
  }
  return request({ action: 'submit', operationId });
}

function render(item, policy) {
  const section = document.createElement('section');
  section.append(element('h2', item.memoryTitle));
  const fields = document.createElement('dl');
  const field = (label, value) => {
    fields.append(element('dt', label), element('dd', value));
  };
  field('Operation', item.operationId);
  field('Memory UUID', item.memoryId);
  field('Memory revision', item.expectedRevision);
  field('Proposed by', item.proposedBy);
  field('Source', item.sourceOrigin === 'host_selected' ? 'Selected by the proposing host' : 'Selected on this device by the owner');
  if (item.sourceLocator) field('Claimed source locator', item.sourceLocator);
  field('Expires', new Date(item.expiresAt * 1000).toLocaleString());
  if (item.file) {
    field('Original filename', item.file.fileName);
    field('Media type', item.inspectedMediaType || item.file.mediaType || 'Awaiting inspection');
    field('Size', item.file.sizeBytes.toLocaleString() + ' bytes');
    field('SHA-256', item.file.sha256);
  }
  if (item.display) {
    field('Embedded metadata', item.embeddedMetadata?.join(', ') || 'None detected');
    field('Garden display', {
      original: 'Original file',
      sanitized_derivative: 'Metadata-stripped derivative',
      withheld_embedded_metadata: 'Withheld because embedded metadata cannot be stripped',
      owner_approved_original: 'Original file with embedded metadata in the owner Garden',
    }[item.display]);
  }
  section.append(fields);
  const actions = document.createElement('div');
  actions.className = 'actions';
  const action = (label, run, className) => {
    const button = element('button', label);
    button.type = 'button';
    if (className) button.className = className;
    button.addEventListener('click', async () => {
      button.disabled = true;
      try {
        const result = await run();
        await load(result.status === 'approved' && result.verification?.verified
          ? 'Attachment approved and verified.'
          : 'Attachment status: ' + result.status.replaceAll('_', ' ') + '.');
      } catch (error) {
        notice.textContent = error.message;
      } finally {
        button.disabled = false;
      }
    });
    actions.append(button);
  };
  const operationId = item.operationId;
  const reviewId = item.reviewId;
  if (item.status === 'source_bytes_unavailable') {
    const file = document.createElement('input');
    file.type = 'file';
    file.accept = 'image/*,audio/*,video/*';
    file.setAttribute('aria-label', 'Select the original media file');
    actions.append(file);
    action('Stage selected original', () => upload(item, file.files[0], policy));
  } else if (item.status === 'uploading') {
    section.append(element('p', 'Waiting for the host to finish staging the original.'));
  } else {
    action(item.status === 'owner_decision_recorded' ? 'Reconcile approval' : 'Approve attachment',
      () => request({ action: 'approve', operationId, reviewId }), 'approve');
    if (item.originalReviewId) {
      action('Approve and display original with metadata',
        () => request({ action: 'approve', operationId, reviewId: item.originalReviewId }));
    }
  }
  action('Reject', () => request({ action: 'reject', operationId, reviewId }));
  section.append(actions);
  return section;
}

async function load(message = '') {
  refresh.disabled = true;
  try {
    const policy = await request();
    items.replaceChildren(...policy.items.map(item => render(item, policy)));
    notice.textContent = message || (policy.items.length ? '' : 'No attachments waiting for review.');
  } catch (error) {
    notice.textContent = error.message;
  } finally {
    refresh.disabled = false;
  }
}
refresh.addEventListener('click', () => load());
await load();
