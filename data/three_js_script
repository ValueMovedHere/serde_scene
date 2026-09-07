(function() {
    if (!window.editor || !window.editor.scene) {
        console.error('❌ 未找到 window.editor.scene');
        return;
    }
    var THREE = window.THREE;
    var scene = window.editor.scene;
    var result = [];
    var debug = [];

    scene.traverse(function(child) {
        if (!child.isMesh || !child.geometry) return;
        var geo = child.geometry;
        var pos = geo.getAttribute('position');
        if (!pos) return;

        if (geo.boundingBox === null) geo.computeBoundingBox();
        var box = geo.boundingBox;
        if (!box) return;

        var size = new THREE.Vector3().copy(box.max).sub(box.min);
        var center = new THREE.Vector3().copy(box.min).add(box.max).multiplyScalar(0.5);
        var count = pos.count;

        var shapeInfo = inferShape(geo, pos, box, size, center, count);
        if (!shapeInfo) {
            debug.push({ name: child.name, vertexCount: count, size: size.toArray() });
            return;
        }

        var requiredKeys = {
            cuboid: ['width', 'height', 'depth'],
            sphere: ['radius'],
            cylinder: ['radius', 'height'],
            cone: ['radius', 'height'],
            capsule: ['radius', 'length']
        }[shapeInfo.type];
        if (!requiredKeys) return;
        for (var k = 0; k < requiredKeys.length; k++) {
            if (shapeInfo.params[requiredKeys[k]] === undefined) {
                shapeInfo.params[requiredKeys[k]] = 1;
            }
        }

        // 类型名大写映射（与 Rust 枚举变体名一致）
var TYPE_MAP = {
    cuboid: 'Cuboid',
    sphere: 'Sphere',
    cylinder: 'Cylinder',
    cone: 'Cone',
    capsule: 'Capsule'
};

// 构建形状枚举对象
var shapeType = TYPE_MAP[shapeInfo.type];
var shapeObj = {};
shapeObj[shapeType] = shapeInfo.params;

result.push({
    shape_type: shapeObj,
    position: child.position.toArray(),
    rotation: child.quaternion.toArray(),
    scale: child.scale.toArray()
});
    });

    console.log(JSON.stringify(result, null, 2));
    console.log('✅ 共导出基本体数量:', result.length);
    if (debug.length > 0) {
        console.warn('⚠️ 以下几何体未能识别（已跳过）:');
        console.table(debug);
    }

    // ============== 核心推断函数 ==============
    function inferShape(geo, pos, box, size, center, count) {
        // 1. 标准几何体检测
        var std = checkStandard(geo);
        if (std) return std;

        // 2. 平面跳过
        if (count === 4) return null;

        // 3. 立方体检测（24 顶点 + 角点验证）
        if (count === 24 && isCuboid(pos, box, size, count)) {
            return { type: 'cuboid', params: { width: size.x, height: size.y, depth: size.z } };
        }

        // 4. 球体检测
        var sphere = detectSphere(pos, center, count);
        if (sphere) return sphere;

        // 5. 圆柱/圆锥检测
        var cyl = detectCylinderCone(pos, box, size, center, count);
        if (cyl) return cyl;

        // 6. 胶囊检测
        var cap = detectCapsule(pos, box, size, center, count);
        if (cap) return cap;

        // 7. 宽松立方体检测（适用于顶点数非 24 的立方体）
        if (isCuboid(pos, box, size, count)) {
            return { type: 'cuboid', params: { width: size.x, height: size.y, depth: size.z } };
        }

        return null;
    }

    function checkStandard(geo) {
        var type = geo.type;
        var p = geo.parameters;
        if (!p) return null;

        if (type === 'BoxGeometry' && p.width !== undefined) {
            return { type: 'cuboid', params: { width: p.width, height: p.height, depth: p.depth } };
        }
        if (type === 'SphereGeometry' && p.radius !== undefined) {
            return { type: 'sphere', params: { radius: p.radius } };
        }
        if (type === 'CylinderGeometry' && p.radiusTop !== undefined) {
            if (p.radiusTop === p.radiusBottom) {
                return { type: 'cylinder', params: { radius: p.radiusTop, height: p.height } };
            } else if (p.radiusTop === 0 || p.radiusBottom === 0) {
                return { type: 'cone', params: { radius: p.radiusTop === 0 ? p.radiusBottom : p.radiusTop, height: p.height } };
            }
        }
        if (type === 'ConeGeometry' && p.radius !== undefined) {
            return { type: 'cone', params: { radius: p.radius, height: p.height } };
        }
        if (type === 'CapsuleGeometry' && p.radius !== undefined) {
            return { type: 'capsule', params: { radius: p.radius, length: p.length } };
        }
        return null;
    }

    function isCuboid(pos, box, size, count) {
        // 提取包围盒 8 个角点
        var corners = [
            [box.min.x, box.min.y, box.min.z],
            [box.min.x, box.min.y, box.max.z],
            [box.min.x, box.max.y, box.min.z],
            [box.min.x, box.max.y, box.max.z],
            [box.max.x, box.min.y, box.min.z],
            [box.max.x, box.min.y, box.max.z],
            [box.max.x, box.max.y, box.min.z],
            [box.max.x, box.max.y, box.max.z]
        ];
        var eps = 0.02 * Math.max(size.x, size.y, size.z, 0.001);
        var cornerHits = new Array(8).fill(0);

        for (var i = 0; i < Math.min(count, 500); i++) {
            var x = pos.getX(i), y = pos.getY(i), z = pos.getZ(i);
            for (var c = 0; c < 8; c++) {
                if (Math.abs(x - corners[c][0]) < eps &&
    Math.abs(y - corners[c][1]) < eps &&
    Math.abs(z - corners[c][2]) < eps) {
                    cornerHits[c]++;
                    break;
                }
            }
        }
        var hitCount = cornerHits.filter(function(h) { return h > 0; }).length;
        return hitCount >= 6;
    }

    function detectSphere(pos, center, count) {
        var sampleCount = Math.min(count, 500);
        var dists = [];
        for (var i = 0; i < sampleCount; i++) {
            var x = pos.getX(i) - center.x;
            var y = pos.getY(i) - center.y;
            var z = pos.getZ(i) - center.z;
            dists.push(Math.sqrt(x*x + y*y + z*z));
        }
        var avg = dists.reduce(function(a, b) { return a + b; }, 0) / dists.length;
        var variance = dists.reduce(function(acc, d) { return acc + (d - avg)*(d - avg); }, 0) / dists.length;
        var stdDev = Math.sqrt(variance);
        if (stdDev / avg < 0.02) {
            return { type: 'sphere', params: { radius: avg } };
        }
        return null;
    }

    function detectCylinderCone(pos, box, size, center, count) {
        var halfH = size.y / 2;
        var topY = center.y + halfH;
        var bottomY = center.y - halfH;
        var topRadii = [], bottomRadii = [];
        var eps = 0.05 * size.y;

        for (var i = 0; i < count; i++) {
            var x = pos.getX(i), y = pos.getY(i), z = pos.getZ(i);
            var r = Math.sqrt(x*x + z*z);
            if (Math.abs(y - topY) < eps) {
                topRadii.push(r);
            }
            if (Math.abs(y - bottomY) < eps) {
                bottomRadii.push(r);
            }
        }

        if (topRadii.length < 3 || bottomRadii.length < 3) return null;

        var avgTopR = topRadii.reduce(function(a, b) { return a + b; }, 0) / topRadii.length;
        var avgBottomR = bottomRadii.reduce(function(a, b) { return a + b; }, 0) / bottomRadii.length;
        var threshold = 0.05 * Math.max(avgTopR, avgBottomR, 0.001);

        if (Math.abs(avgTopR - avgBottomR) < threshold) {
            return { type: 'cylinder', params: { radius: (avgTopR + avgBottomR) / 2, height: size.y } };
        } else if (avgTopR < threshold) {
            return { type: 'cone', params: { radius: avgBottomR, height: size.y } };
        } else if (avgBottomR < threshold) {
            return { type: 'cone', params: { radius: avgTopR, height: size.y } };
        }
        return null;
    }

    function detectCapsule(pos, box, size, center, count) {
        var halfH = size.y / 2;
        var topY = center.y + halfH;
        var bottomY = center.y - halfH;
        var topCapDists = [], bottomCapDists = [];
        var eps = 0.1 * size.y;

        for (var i = 0; i < count; i++) {
            var x = pos.getX(i), y = pos.getY(i), z = pos.getZ(i);
            if (y > topY - eps) {
                var dy = y - (topY - halfH);
                var d = Math.sqrt(x*x + dy*dy + z*z);
                topCapDists.push(d);
            }
            if (y < bottomY + eps) {
                var dy = y - (bottomY + halfH);
                var d = Math.sqrt(x*x + dy*dy + z*z);
                bottomCapDists.push(d);
            }
        }

        if (topCapDists.length < 5 || bottomCapDists.length < 5) return null;

        var avgTopD = topCapDists.reduce(function(a, b) { return a + b; }, 0) / topCapDists.length;
        var avgBottomD = bottomCapDists.reduce(function(a, b) { return a + b; }, 0) / bottomCapDists.length;
        var topVar = topCapDists.reduce(function(acc, d) { return acc + (d - avgTopD)*(d - avgTopD); }, 0) / topCapDists.length;
        var bottomVar = bottomCapDists.reduce(function(acc, d) { return acc + (d - avgBottomD)*(d - avgBottomD); }, 0) / bottomCapDists.length;
        var topStd = Math.sqrt(topVar);
        var bottomStd = Math.sqrt(bottomVar);

        if (topStd / avgTopD < 0.05 && bottomStd / avgBottomD < 0.05 &&
            Math.abs(avgTopD - avgBottomD) / Math.max(avgTopD, avgBottomD, 0.001) < 0.1) {
            var radius = (avgTopD + avgBottomD) / 2;
            var length = size.y - 2 * radius;
            if (length > 0) {
                return { type: 'capsule', params: { radius: radius, length: length } };
            }
        }
        return null;
    }
})();
