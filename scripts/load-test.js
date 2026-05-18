import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('error_rate');
const latencyTrend = new Trend('latency');
const requestsPerSecond = new Counter('rps');

// Test configuration
export const options = {
  stages: [
    { duration: '30s', target: 10 },   // Ramp up to 10 users
    { duration: '1m', target: 50 },    // Ramp up to 50 users
    { duration: '2m', target: 100 },   // Ramp up to 100 users
    { duration: '1m', target: 100 },   // Stay at 100 users
    { duration: '30s', target: 0 },    // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests should be below 500ms
    error_rate: ['rate<0.01'],        // Error rate should be less than 1%
    http_req_failed: ['rate<0.01'],   // Failed requests should be less than 1%
  },
};

// Base URL - can be overridden with environment variable
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

// Test data
const testUrls = [
  'https://example.com',
  'https://github.com',
  'https://google.com',
  'https://stackoverflow.com',
  'https://reddit.com',
  'https://twitter.com',
  'https://linkedin.com',
  'https://medium.com',
  'https://dev.to',
  'https://hashnode.com',
];

// Generate random short code
function generateRandomCode() {
  return Math.random().toString(36).substring(2, 8);
}

export default function () {
  // Test 1: Health check
  const healthRes = http.get(`${BASE_URL}/health`);
  check(healthRes, {
    'health check status is 200': (r) => r.status === 200,
    'health check response time < 100ms': (r) => r.timings.duration < 100,
  });

  // Test 2: Create short URL
  const randomUrl = testUrls[Math.floor(Math.random() * testUrls.length)];
  const shortenPayload = JSON.stringify({ url: randomUrl });

  const shortenRes = http.post(`${BASE_URL}/shorten`, shortenPayload, {
    headers: { 'Content-Type': 'application/json' },
  });

  check(shortenRes, {
    'shorten status is 200': (r) => r.status === 200,
    'shorten response has short_code': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.short_code !== undefined;
      } catch {
        return false;
      }
    },
    'shorten response time < 200ms': (r) => r.timings.duration < 200,
  });

  errorRate.add(shortenRes.status !== 200);
  latencyTrend.add(shortenRes.timings.duration);
  requestsPerSecond.add(1);

  // Test 3: Redirect (if shorten was successful)
  if (shortenRes.status === 200) {
    try {
      const shortenBody = JSON.parse(shortenRes.body);
      const shortCode = shortenBody.short_code;

      const redirectRes = http.get(`${BASE_URL}/${shortCode}`, {
        redirects: 0,
      });

      check(redirectRes, {
        'redirect status is 301 or 302': (r) => r.status === 301 || r.status === 302,
        'redirect response time < 100ms': (r) => r.timings.duration < 100,
      });

      errorRate.add(redirectRes.status !== 301 && redirectRes.status !== 302);
      latencyTrend.add(redirectRes.timings.duration);
    } catch {
      // Ignore JSON parsing errors
    }
  }

  // Test 4: Get stats for random code
  const randomCode = generateRandomCode();
  const statsRes = http.get(`${BASE_URL}/${randomCode}/stats`);

  // Stats might return 404 for non-existent codes, which is expected
  check(statsRes, {
    'stats response time < 200ms': (r) => r.timings.duration < 200,
  });

  // Test 5: Metrics endpoint
  const metricsRes = http.get(`${BASE_URL}/metrics`);
  check(metricsRes, {
    'metrics status is 200': (r) => r.status === 200,
  });

  // Sleep to simulate user think time
  sleep(Math.random() * 2 + 1); // Random sleep between 1-3 seconds
}

// Setup function - runs once per VU
export function setup() {
  console.log('Starting load test...');
  console.log(`Target URL: ${BASE_URL}`);

  // Verify the service is up
  const healthRes = http.get(`${BASE_URL}/health`);
  if (healthRes.status !== 200) {
    console.error('Health check failed! Service may not be ready.');
    return;
  }

  console.log('Service is healthy, starting load test...');
}

// Teardown function - runs once after all VUs complete
export function teardown() {
  console.log('Load test completed.');
}