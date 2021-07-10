class FrustumDDA
{
public:
// Constructor
FrustumDDA(const openvdb::math::NonlinearFrustumMap& map)
: m_map(map) {}
// Initialise traversal
bool init(openvdb::math::Ray& ray, double tmax);
// Take step
bool step(double& t, openvdb::Coord& ijk);
private:
double m_tn; // Distance traversed so far
double m_tM; // Maximum traversal distance
openvdb::Coord m_ijk; // Current leaf coordinates
openvdb::Vec3i m_dijk; // Coordinate updates per step
openvdb::Vec3d m_t; // Per-axis intersection distances
openvdb::Vec3d m_p, m_q, m_dp, m_dq; // Stepping variables
const openvdb::math::NonlinearFrustumMap& m_map; // The frustum
};

bool FrustumDDA::step(double& t, openvdb::Coord& ijk)
{
// Check end of traversal
if (m_tn < m_tM)
{
// Find axis with nearest intersection
size_t a = openvdb::math::MinIndex(m_t);
// Pass current traversal state to the outside,
// also updating the distance travelled so far
t = m_tn = std::min(m_t[a], m_tM);
ijk = m_ijk;
// Take step along axis ’a’
// To avoid branching in the K axis case,
// init() sets m q[2] = 1.0 and m dq[2] = 0.0
m_p[a] += m_dp[a];
m_q[a] -= m_dq[a];
double ta = m_p[a]/m_q[a];
// An invalid increment, to occur,
// will only be on the I or J axes
assert(ta > 0.0 || a < 2);
m_t[a] = (ta > 0.0) ?
ta : std::numeric_limits<double>::infinity();
// Update leaf coordinates with increments from eqs. 7
m_ijk[a] += m_dijk[a];
return true;
}
return false;
}
