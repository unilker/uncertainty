// Plain-language, step-by-step explanation of a project, in English or
// Turkish, written from the project data itself: what is measured, what each
// input means and how its standard uncertainty follows from the given
// numbers, what the program computes and which results the source expects.
// Used for the library examples and for the user's own projects. Pure Dart
// (no Flutter), so that tool/write_learning_docs.dart can use it too.

import 'dart:math' as math;

import '../format.dart';

/// Markdown explaining [project] in [lang] ('en' or 'tr').
String explainProject(Map<String, dynamic> project, String lang) => _Explainer(project, lang).run();

class _Explainer {
  _Explainer(this.p, this.lang);
  final Map<String, dynamic> p;
  final String lang;
  final b = StringBuffer();

  bool get tr => lang == 'tr';
  String t(String en, String trText) => tr ? trText : en;

  /// A given number, as typed (at most about ten characters).
  String g(Object? x) {
    if (x is! num) return '—';
    final s = fmtGivenIn(x, lang);
    return s.length > 10 ? fmtNum(x, 5, lang) : s;
  }

  /// A whole number with thin spaces between thousands (1 000 000).
  String whole(num x) => x.round().toString().replaceAllMapped(RegExp(r'\B(?=(\d{3})+(?!\d))'), (_) => '\u202f');

  /// A computed number with [sig] significant digits.
  String n(num x, [int sig = 3]) => fmtNum(x, sig, lang);

  String text(Object? v) {
    if (v is Map) return (v[lang] ?? v['en'] ?? '').toString();
    return v?.toString() ?? '';
  }

  String unit(Object? u) => (u == null || u.toString().isEmpty) ? '' : ' ${u.toString()}';

  void h(String s) => b
    ..writeln()
    ..writeln('### $s')
    ..writeln();
  void para(String s) => b
    ..writeln(s)
    ..writeln();
  void item(String s) => b.writeln('- $s');
  void numbered(int i, String s) => b.writeln('$i. $s');
  void tip(String s) => b
    ..writeln()
    ..writeln('> **${t('Try', 'Deneyin')}:** $s')
    ..writeln();

  String run() {
    final meta = (p['metadata'] as Map?) ?? const {};
    final description = text(meta['description']);
    if (description.isNotEmpty) {
      h(t('What is this project about?', 'Bu proje ne hakkında?'));
      para(description);
    }
    if (p['model'] is Map) {
      _model(p['model'] as Map, multi: false);
    } else if (p['multivariate'] is Map) {
      _model(p['multivariate'] as Map, multi: true);
    } else if (p['topdown'] is Map) {
      _topdown(p['topdown'] as Map);
    } else if (p['proficiency'] is Map) {
      _proficiency(p['proficiency'] as Map);
    } else if (p['calibration'] is Map) {
      _calibration(p['calibration'] as Map);
    } else if (p['precision_experiment'] is Map) {
      _precision(p['precision_experiment'] as Map);
    } else if (p['control_chart'] is Map) {
      _controlChart(p['control_chart'] as Map);
    } else if (p['sampling'] is Map) {
      _sampling(p['sampling'] as Map);
    } else if (p['error_function'] is Map) {
      _errorFunction(p['error_function'] as Map);
    } else if (p['consensus'] is Map) {
      _consensus(p['consensus'] as Map);
    } else if (p['multicomponent_conformity'] is Map) {
      _multicomponent(p['multicomponent_conformity'] as Map);
    }
    if (p['conformity'] is Map) _conformity(p['conformity'] as Map, standalone: p['model'] == null);
    _expected();
    return b.toString().trim();
  }

  // --- Measurement models --------------------------------------------------------------------------------------

  void _model(Map m, {required bool multi}) {
    final outputs = multi ? ((m['outputs'] as List?) ?? const []) : [m['output']];
    h(t('Step 1. What do we measure, and how is it calculated?', 'Adım 1. Ne ölçüyoruz ve nasıl hesaplanıyor?'));
    if (multi) {
      para(
        t(
          'This project has ${outputs.length} results (output quantities) that are calculated together from the same inputs. Each has its own equation:',
          'Bu projede aynı girdilerden birlikte hesaplanan ${outputs.length} sonuç (çıktı büyüklüğü) var. Her birinin kendi denklemi var:',
        ),
      );
    }
    for (final o in outputs) {
      if (o is! Map) continue;
      final d = text(o['description']);
      final eq = o['expression']?.toString() ?? '';
      if (!multi) {
        para(
          t(
            'The result is **${o['name']}**${o['unit'] != null ? ' in ${o['unit']}' : ''}${d.isEmpty ? '' : ' ($d)'}. It is not measured directly: it is calculated from the input quantities with the measurement model',
            'Sonuç **${o['name']}**${o['unit'] != null ? ' (${o['unit']})' : ''}${d.isEmpty ? '' : ': $d'}. Doğrudan ölçülmez; girdi büyüklüklerinden şu ölçüm modeliyle hesaplanır',
          ),
        );
        b
          ..writeln('```')
          ..writeln('${o['name']} = $eq')
          ..writeln('```')
          ..writeln();
      } else {
        item('**${o['name']}**${unit(o['unit'])}: `${o['name']} = $eq`');
      }
    }
    if (multi) b.writeln();
    if (m['implicit'] == true) {
      para(
        t(
          'The model is *implicit*: the equations are not solved for the results, so the program solves them numerically.',
          'Model *kapalıdır*: denklemler sonuçlara göre çözülmüş değildir; program onları sayısal olarak çözer.',
        ),
      );
    }
    final inter = (m['intermediates'] as List?) ?? const [];
    if (inter.isNotEmpty) {
      para(
        t(
          'To keep the equation readable, ${inter.length} intermediate quantities are calculated first:',
          'Denklemi okunur tutmak için önce ${inter.length} ara büyüklük hesaplanır:',
        ),
      );
      for (final i in inter) {
        if (i is Map) item('`${i['name']} = ${i['expression']}`');
      }
      b.writeln();
    }
    final lower = multi ? null : (m['output'] as Map?)?['lower_bound'];
    final upper = multi ? null : (m['output'] as Map?)?['upper_bound'];
    if (lower != null || upper != null) {
      para(
        t(
          'The result cannot be ${lower != null ? 'below ${g(lower)}' : ''}${lower != null && upper != null ? ' or ' : ''}${upper != null ? 'above ${g(upper)}' : ''} (a physical bound, e.g. a purity cannot exceed 1). The program respects this bound when it states the interval.',
          'Sonuç ${lower != null ? '${g(lower)} değerinin altında' : ''}${lower != null && upper != null ? ' ya da ' : ''}${upper != null ? '${g(upper)} değerinin üstünde' : ''} olamaz (fiziksel bir sınır; örneğin saflık 1\'i aşamaz). Program aralığı verirken bu sınırı dikkate alır.',
        ),
      );
    }

    final inputs = (m['inputs'] as List?) ?? const [];
    h(t('Step 2. The input quantities and their uncertainties', 'Adım 2. Girdi büyüklükleri ve belirsizlikleri'));
    para(
      t(
        'The model has ${inputs.length} inputs. For each one we need a best value (estimate) and a **standard uncertainty** u, which says how far the true value could plausibly be from the estimate. Where the number comes from decides how u is obtained:',
        'Modelde ${inputs.length} girdi var. Her biri için bir en iyi değer (tahmin) ve gerçek değerin bu tahminden ne kadar uzakta olabileceğini anlatan bir **standart belirsizlik** u gerekir. Sayının nereden geldiği, u\'nun nasıl bulunacağını belirler:',
      ),
    );
    var k = 0;
    for (final i in inputs) {
      if (i is! Map) continue;
      k++;
      final d = text(i['description']);
      final dist = (i['distribution'] as Map?) ?? const {};
      numbered(k, '**${i['name']}**${d.isEmpty ? '' : ': $d'}. ${_distribution(dist, i['unit']?.toString())}${_dof(i['dof'])}');
    }
    b.writeln();
    final corr = (m['correlations'] as List?) ?? const [];
    if (corr.isNotEmpty) {
      para(
        t(
          '**Correlated inputs.** Some inputs are not independent, because a common effect (the same standard, the same instrument, the same temperature) acts on both. The correlation coefficient r tells how strongly; the program adds the corresponding term to the combined uncertainty:',
          '**Korelasyonlu girdiler.** Bazı girdiler birbirinden bağımsız değildir; her ikisine de ortak bir etki (aynı standart, aynı cihaz, aynı sıcaklık) etki eder. Korelasyon katsayısı r bunun ne kadar güçlü olduğunu söyler; program birleşik belirsizliğe karşılık gelen terimi ekler:',
        ),
      );
      for (final c in corr) {
        if (c is Map) item('${c['a']} ↔ ${c['b']}: r = ${g(c['r'])}');
      }
      b.writeln();
    }

    final a = (p['analysis'] as Map?) ?? const {};
    final gum = (a['gum'] as Map?) ?? const {};
    final mcm = a['mcm'] as Map?;
    h(t('Step 3. What the program calculates', 'Adım 3. Program ne hesaplıyor?'));
    numbered(
      1,
      t(
        'It puts the estimates of the inputs into the equation: this gives the estimate of the result.',
        'Girdilerin tahminlerini denklemde yerine koyar: bu, sonucun tahminini verir.',
      ),
    );
    numbered(
      2,
      t(
        'For each input it finds the **sensitivity coefficient** c: how much the result changes when that input changes by one unit (the derivative of the equation).',
        'Her girdi için **duyarlılık katsayısını** c bulur: o girdi bir birim değişince sonuç ne kadar değişir (denklemin türevi).',
      ),
    );
    numbered(
      3,
      t(
        'It multiplies each standard uncertainty by its sensitivity coefficient: the **contribution** c·u of the input, in the unit of the result.',
        'Her standart belirsizliği duyarlılık katsayısıyla çarpar: girdinin sonuç biriminden **katkısı** c·u.',
      ),
    );
    numbered(
      4,
      t(
        'It combines the contributions as the square root of the sum of their squares (plus the correlation terms): the **combined standard uncertainty** u_c.',
        'Katkıları, karelerinin toplamının karekökü olarak birleştirir (korelasyon terimleriyle birlikte): **birleşik standart belirsizlik** u_c.',
      ),
    );
    final kf = gum['coverage_factor'];
    final pr = gum['coverage_probability'];
    numbered(
      5,
      kf != null
          ? t(
              'It multiplies u_c by the coverage factor k = ${g(kf)}: the **expanded uncertainty** U = k·u_c, which gives an interval that contains the true value with a high probability (about 95 % for k = 2).',
              'u_c\'yi kapsam faktörü k = ${g(kf)} ile çarpar: **genişletilmiş belirsizlik** U = k·u_c. Bu, gerçek değeri yüksek olasılıkla içeren bir aralık verir (k = 2 için yaklaşık %95).',
            )
          : pr != null
          ? t(
              'It chooses the coverage factor k for a coverage probability of ${fmtPct(pr, 3, lang)}, using the effective degrees of freedom (Welch–Satterthwaite), and gives the **expanded uncertainty** U = k·u_c.',
              'Kapsam faktörü k\'yi ${fmtPct(pr as num, 3, lang)} kapsam olasılığı için, etkin serbestlik derecesini (Welch–Satterthwaite) kullanarak seçer ve **genişletilmiş belirsizliği** U = k·u_c verir.',
            )
          : t(
              'It gives the **expanded uncertainty** U = k·u_c with the coverage factor of the settings.',
              'Ayarlardaki kapsam faktörüyle **genişletilmiş belirsizliği** U = k·u_c verir.',
            ),
    );
    if (gum['higher_order'] == true) {
      numbered(
        6,
        t(
          'Because the model is clearly nonlinear, second-order terms are added (GUM 5.1.2 and its 2026 amendment).',
          'Model belirgin biçimde doğrusal olmadığı için ikinci mertebe terimler eklenir (GUM 5.1.2 ve 2026 değişikliği).',
        ),
      );
    }
    b.writeln();
    if (mcm != null) {
      final trials = mcm['trials'];
      para(
        t(
          'It also uses the **Monte Carlo method**: it draws ${trials is num ? whole(trials) : 'many'} random values of every input from its distribution, calculates the result for each draw, and reads the uncertainty and the coverage interval directly from the ${trials is num ? whole(trials) : ''} results. If the two methods agree, the simpler GUM result can be trusted.',
          'Ayrıca **Monte Carlo yöntemini** kullanır: her girdinin dağılımından ${trials is num ? whole(trials) : 'çok sayıda'} rastgele değer çeker, her çekiliş için sonucu hesaplar ve belirsizliği ile kapsam aralığını doğrudan bu ${trials is num ? whole(trials) : ''} sonuçtan okur. İki yöntem uyuşuyorsa daha basit olan GUM sonucuna güvenilebilir.',
        ),
      );
    }
    tip(
      t(
        'Press **Calculate**, then open **Results**. In the uncertainty budget, the column *Share of variance* shows which input matters most: improving that input is the most effective way to reduce U. Change its uncertainty in the **Model** tab and calculate again to see the effect.',
        '**Hesapla**\'ya basın ve **Sonuçlar**\'ı açın. Belirsizlik bütçesindeki *varyans payı* sütunu hangi girdinin en çok etkilediğini gösterir: U\'yu azaltmanın en etkili yolu o girdiyi iyileştirmektir. **Model** sekmesinde onun belirsizliğini değiştirip yeniden hesaplayarak etkisini görün.',
      ),
    );
  }

  String _dof(Object? dof) {
    if (dof is! num) return '';
    return t(
      ' This standard uncertainty is itself only known roughly: it has ${g(dof)} degrees of freedom.',
      ' Bu standart belirsizliğin kendisi de yaklaşık bilinir: ${g(dof)} serbestlik derecesi vardır.',
    );
  }

  /// Limits of a symmetric distribution: (centre, half-width) from value and
  /// half_width, or from lower and upper.
  (num?, num?, num?, num?) _limits(Map d) {
    final a = d['half_width'];
    final v = d['value'];
    if (a is num) return (v is num ? v : 0, a, (v is num ? v : 0) - a, (v is num ? v : 0) + a);
    final lo = d['lower'];
    final hi = d['upper'];
    if (lo is num && hi is num) return ((lo + hi) / 2, (hi - lo) / 2, lo, hi);
    return (v is num ? v : null, null, null, null);
  }

  String _distribution(Map d, String? u) {
    final un = unit(u);
    switch (d['type']) {
      case 'normal':
        if (d['sd'] is num) {
          return t(
            'Best value ${g(d['mean'])}$un with standard uncertainty u = ${g(d['sd'])}$un (normal distribution: values near the estimate are the most likely; typical for a calibration certificate or for the mean of many readings).',
            'En iyi değer ${g(d['mean'])}$un, standart belirsizlik u = ${g(d['sd'])}$un (normal dağılım: tahmine yakın değerler en olasıdır; kalibrasyon sertifikaları ya da çok sayıda okumanın ortalaması için tipiktir).',
          );
        }
        final e = d['expanded'];
        final k = d['k'];
        if (e is num && k is num) {
          return t(
            'Best value ${g(d['mean'])}$un. The source (e.g. a certificate) gives the expanded uncertainty U = ${g(e)}$un with k = ${g(k)}, so the standard uncertainty is u = U/k = ${g(e)}/${g(k)} = ${n(e / k)}$un (normal distribution).',
            'En iyi değer ${g(d['mean'])}$un. Kaynak (örneğin bir sertifika) genişletilmiş belirsizliği U = ${g(e)}$un, k = ${g(k)} olarak verir; bu yüzden standart belirsizlik u = U/k = ${g(e)}/${g(k)} = ${n(e / k)}$un olur (normal dağılım).',
          );
        }
        return t('Normal distribution with best value ${g(d['mean'])}$un.', 'En iyi değeri ${g(d['mean'])}$un olan normal dağılım.');
      case 't':
        final nu = d['dof'];
        num? s = d['scale'] is num ? d['scale'] as num : null;
        var how = '';
        if (s == null && d['expanded'] is num && d['k'] is num) {
          s = (d['expanded'] as num) / (d['k'] as num);
          how = t(' (U = ${g(d['expanded'])} divided by k = ${g(d['k'])})', ' (U = ${g(d['expanded'])}, k = ${g(d['k'])} ile bölünerek)');
        }
        return t(
          'Best value ${g(d['mean'])}$un with standard uncertainty u = ${s == null ? '—' : n(s)}$un$how. Because it is based on few data (ν = ${g(nu)} degrees of freedom), a Student t distribution is used: it has heavier tails than the normal distribution and makes the coverage factor larger.',
          'En iyi değer ${g(d['mean'])}$un, standart belirsizlik u = ${s == null ? '—' : n(s)}$un$how. Az sayıda veriye dayandığı için (ν = ${g(nu)} serbestlik derecesi) Student t dağılımı kullanılır: kuyrukları normal dağılımdan kalındır ve kapsam faktörünü büyütür.',
        );
      case 'rectangular':
        final (c, a, lo, hi) = _limits(d);
        if (a == null) return t('Rectangular distribution.', 'Dikdörtgen dağılım.');
        return t(
          'Only limits are known: the value lies between ${g(lo)} and ${g(hi)}$un (${g(c)} ± ${g(a)}), and no value inside is more likely than another (rectangular distribution; typical for a tolerance, a specification, the resolution of a display or a drift limit). The standard uncertainty is u = a/√3 = ${g(a)}/√3 = ${n(a / math.sqrt(3))}$un.',
          'Yalnızca sınırlar biliniyor: değer ${g(lo)} ile ${g(hi)}$un arasında (${g(c)} ± ${g(a)}) ve aradaki hiçbir değer diğerinden daha olası değil (dikdörtgen dağılım; tolerans, üretici spesifikasyonu, gösterge çözünürlüğü ya da sürüklenme sınırı için tipiktir). Standart belirsizlik u = a/√3 = ${g(a)}/√3 = ${n(a / math.sqrt(3))}$un.',
        );
      case 'triangular':
        final (c, a, lo, hi) = _limits(d);
        if (a == null) return t('Triangular distribution.', 'Üçgen dağılım.');
        return t(
          'The value lies between ${g(lo)} and ${g(hi)}$un, and values near the centre ${g(c)} are more likely than values near the limits (triangular distribution). u = a/√6 = ${g(a)}/√6 = ${n(a / math.sqrt(6))}$un.',
          'Değer ${g(lo)} ile ${g(hi)}$un arasında ve merkez ${g(c)}\'ye yakın değerler sınırlara yakın değerlerden daha olası (üçgen dağılım). u = a/√6 = ${g(a)}/√6 = ${n(a / math.sqrt(6))}$un.',
        );
      case 'arcsine':
        final (c, a, lo, hi) = _limits(d);
        if (a == null) return t('U-shaped (arcsine) distribution.', 'U biçimli (arksinüs) dağılım.');
        return t(
          'The value swings between ${g(lo)} and ${g(hi)}$un and is most often near the limits (U-shaped or arcsine distribution; e.g. a temperature cycling around a set point, or a mismatch phase). u = a/√2 = ${g(a)}/√2 = ${n(a / math.sqrt(2))}$un.',
          'Değer ${g(lo)} ile ${g(hi)}$un arasında salınır ve çoğunlukla sınırlara yakındır (U biçimli ya da arksinüs dağılım; örneğin bir ayar noktası etrafında salınan sıcaklık ya da uyumsuzluk fazı). u = a/√2 = ${g(a)}/√2 = ${n(a / math.sqrt(2))}$un.',
        );
      case 'trapezoidal':
        final (c, a, lo, hi) = _limits(d);
        final beta = (d['beta'] as num?) ?? 0;
        if (a == null) return t('Trapezoidal distribution.', 'Yamuk dağılım.');
        return t(
          'The value lies between ${g(lo)} and ${g(hi)}$un; values in the middle part (a fraction β = ${g(beta)} of the width) are equally likely, and the likelihood falls towards the limits (trapezoidal distribution). u = a·√((1 + β²)/6) = ${n(a * math.sqrt((1 + beta * beta) / 6))}$un.',
          'Değer ${g(lo)} ile ${g(hi)}$un arasında; ortadaki bölgedeki (genişliğin β = ${g(beta)} kesri) değerler eşit olasılıklı, sınırlara doğru olasılık azalıyor (yamuk dağılım). u = a·√((1 + β²)/6) = ${n(a * math.sqrt((1 + beta * beta) / 6))}$un.',
        );
      case 'curvilinear_trapezoid':
        final (c, a, lo, hi) = _limits(d);
        final dd = (d['d'] as num?) ?? 0;
        if (a == null) return t('Rectangular distribution with inexact limits.', 'Sınırları tam bilinmeyen dikdörtgen dağılım.');
        return t(
          'The value lies between ${g(lo)} and ${g(hi)}$un, but the limits themselves are only known to ± ${g(dd)} (curvilinear trapezoid, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = ${n(math.sqrt(a * a / 3 + dd * dd / 9))}$un.',
          'Değer ${g(lo)} ile ${g(hi)}$un arasında, ancak sınırların kendisi de ± ${g(dd)} belirsiz (eğrisel yamuk, JCGM 101 6.4.3). u = √(a²/3 + d²/9) = ${n(math.sqrt(a * a / 3 + dd * dd / 9))}$un.',
        );
      case 'exponential':
        return t(
          'Only the mean ${g(d['mean'])}$un of a non-negative quantity is known (exponential distribution; u equals the mean).',
          'Negatif olamayan bir büyüklüğün yalnızca ortalaması ${g(d['mean'])}$un biliniyor (üstel dağılım; u ortalamaya eşittir).',
        );
      case 'gamma':
        final sh = d['shape'];
        final sc = d['scale'];
        if (sh is num && sc is num) {
          return t(
            'Gamma distribution (a non-negative, skewed quantity) with mean ${n(sh * sc)}$un and u = ${n(math.sqrt(sh) * sc)}$un.',
            'Gama dağılımı (negatif olamayan, çarpık bir büyüklük): ortalama ${n(sh * sc)}$un, u = ${n(math.sqrt(sh) * sc)}$un.',
          );
        }
        return t('Gamma distribution.', 'Gama dağılımı.');
      case 'beta':
        final al = d['alpha'];
        final be = d['beta'];
        final lo = (d['lower'] as num?) ?? 0;
        final hi = (d['upper'] as num?) ?? 1;
        if (al is num && be is num) {
          final s = al + be;
          return t(
            'Beta distribution between ${g(lo)} and ${g(hi)}$un (a quantity confined to an interval, possibly asymmetric, such as a purity close to 1): mean ${n(lo + (hi - lo) * al / s, 6)}$un, u = ${n((hi - lo) * math.sqrt(al * be / (s * s * (s + 1))))}$un.',
            '${g(lo)} ile ${g(hi)}$un arasında beta dağılımı (bir aralığa hapsolmuş, asimetrik olabilen bir büyüklük; örneğin 1\'e yakın bir saflık): ortalama ${n(lo + (hi - lo) * al / s, 6)}$un, u = ${n((hi - lo) * math.sqrt(al * be / (s * s * (s + 1))))}$un.',
          );
        }
        return t(
          'Beta distribution between ${g(lo)} and ${g(hi)}$un with mean ${g(d['mean'])} and u = ${g(d['sd'])}$un (a quantity confined to an interval).',
          '${g(lo)} ile ${g(hi)}$un arasında beta dağılımı; ortalama ${g(d['mean'])}, u = ${g(d['sd'])}$un (bir aralığa hapsolmuş büyüklük).',
        );
      case 'type_a':
        final obs = [for (final x in (d['observations'] as List?) ?? const []) (x as num).toDouble()];
        if (obs.length < 2) return t('Repeated observations (Type A evaluation).', 'Tekrarlı gözlemler (A tipi değerlendirme).');
        final nn = obs.length;
        final mean = obs.reduce((x, y) => x + y) / nn;
        final s = math.sqrt(obs.map((x) => (x - mean) * (x - mean)).reduce((x, y) => x + y) / (nn - 1));
        return t(
          'Measured $nn times (a **Type A** evaluation, from statistics). The estimate is the mean ${n(mean, 6)}$un; the readings scatter with a standard deviation s = ${n(s)}$un, and the standard uncertainty of the mean is u = s/√n = ${n(s)}/√$nn = ${n(s / math.sqrt(nn))}$un, with ν = n − 1 = ${nn - 1} degrees of freedom.',
          '$nn kez ölçülmüş (**A tipi** değerlendirme, istatistikten). Kestirim ortalama ${n(mean, 6)}$un; okumalar s = ${n(s)}$un standart sapmayla dağılır ve ortalamanın standart belirsizliği u = s/√n = ${n(s)}/√$nn = ${n(s / math.sqrt(nn))}$un, serbestlik derecesi ν = n − 1 = ${nn - 1}.',
        );
      case 'type_a_pooled':
        final sp = d['pooled_sd'];
        final nn = d['n'];
        return t(
          'Mean of ${g(nn)} readings, ${g(d['mean'])}$un. Instead of the scatter of these few readings, a standard deviation s_p = ${g(sp)}$un determined earlier from more data (ν = ${g(d['pooled_dof'])}) is used, so u = s_p/√n = ${sp is num && nn is num ? n(sp / math.sqrt(nn)) : '—'}$un.',
          '${g(nn)} okumanın ortalaması, ${g(d['mean'])}$un. Bu birkaç okumanın dağılımı yerine daha önce daha çok veriden belirlenmiş bir standart sapma s_p = ${g(sp)}$un (ν = ${g(d['pooled_dof'])}) kullanılır; böylece u = s_p/√n = ${sp is num && nn is num ? n(sp / math.sqrt(nn)) : '—'}$un.',
        );
      case 'constant':
        return t(
          'An exact value, ${g(d['value'])}$un, with no uncertainty (for example a defined constant or a nominal value).',
          'Tam bilinen bir değer, ${g(d['value'])}$un; belirsizliği yok (örneğin tanımlı bir sabit ya da anma değeri).',
        );
      default:
        return '';
    }
  }

  // --- Other project types --------------------------------------------------------------------------------------

  void _topdown(Map td) {
    final prec = (td['precision'] as List?) ?? const [];
    final comps = (td['components'] as List?) ?? const [];
    h(t('Step 1. The idea: uncertainty from the laboratory\'s own data', 'Adım 1. Fikir: laboratuvarın kendi verisinden belirsizlik'));
    para(
      t(
        'This is a **top-down** evaluation of **${td['quantity'] ?? ''}**${td['unit'] != null ? ' (${td['unit']})' : ''}. Instead of modelling every step of the method, it uses data that the laboratory already has: how much the results scatter when the method is repeated on different days (precision) and how far they are, on average, from the true value (bias or recovery). Together these cover most sources of uncertainty at once.',
        'Bu, **${td['quantity'] ?? ''}**${td['unit'] != null ? ' (${td['unit']})' : ''} için **yukarıdan aşağı** (top-down) bir değerlendirmedir. Yöntemin her adımını modellemek yerine laboratuvarın zaten sahip olduğu verileri kullanır: yöntem farklı günlerde tekrarlandığında sonuçların ne kadar dağıldığı (kesinlik) ve ortalamada gerçek değerden ne kadar uzak oldukları (sapma ya da geri kazanım). Bu ikisi birlikte belirsizlik kaynaklarının çoğunu bir kerede kapsar.',
      ),
    );
    final levels = (td['levels'] as List?) ?? const [];
    if (levels.isNotEmpty) {
      para(
        t(
          'The uncertainty is evaluated at ${levels.length} concentration levels: ${levels.map(g).join('; ')}${unit(td['unit'])}.',
          'Belirsizlik ${levels.length} derişim düzeyinde değerlendirilir: ${levels.map(g).join('; ')}${unit(td['unit'])}.',
        ),
      );
    }
    h(t('Step 2. The data', 'Adım 2. Veriler'));
    if (prec.isNotEmpty) {
      para(
        t('**Precision** (random effects), ${prec.length} entr${prec.length == 1 ? 'y' : 'ies'}:', '**Kesinlik** (rastgele etkiler), ${prec.length} kayıt:'),
      );
      for (final e in prec) {
        if (e is! Map) continue;
        final data = (e['data'] as Map?) ?? const {};
        item('**${e['name'] ?? ''}**: ${_precisionData(data)}');
      }
      b.writeln();
    }
    final bias = (td['bias'] as List?) ?? (td['bias'] is Map ? [td['bias']] : const []);
    final rec = td['recovery'];
    if (bias.isNotEmpty) {
      para(
        t(
          '**Bias** (systematic effects): ${bias.length} source${bias.length == 1 ? '' : 's'}, e.g. certified reference materials or proficiency tests. From the differences between the laboratory\'s results and the reference values, the program estimates u(bias), which includes the uncertainty of the reference values themselves.',
          '**Sapma** (sistematik etkiler): ${bias.length} kaynak; örneğin sertifikalı referans malzemeler ya da yeterlilik testleri. Program, laboratuvar sonuçlarıyla referans değerler arasındaki farklardan u(sapma)\'yı kestirir; buna referans değerlerin kendi belirsizliği de dahildir.',
        ),
      );
    }
    if (rec is Map) {
      final tests = (rec['tests'] as List?) ?? const [];
      para(
        t(
          '**Recovery**: ${tests.length} recovery test${tests.length == 1 ? '' : 's'} (reference materials or spiked samples). The mean recovery R and its uncertainty describe how much of the analyte the method finds; if R differs significantly from 100 %, a correction is considered.',
          '**Geri kazanım**: ${tests.length} geri kazanım testi (referans malzemeler ya da katkılanmış numuneler). Ortalama geri kazanım R ve belirsizliği, yöntemin analitin ne kadarını bulduğunu anlatır; R %100\'den anlamlı biçimde farklıysa düzeltme düşünülür.',
        ),
      );
    }
    if (comps.isNotEmpty) {
      para(t('**Further components**, added as they are given:', '**Diğer bileşenler**, verildiği gibi eklenir:'));
      for (final c in comps) {
        if (c is Map) item('${c['name'] ?? ''}');
      }
      b.writeln();
    }
    h(t('Step 3. What the program calculates', 'Adım 3. Program ne hesaplıyor?'));
    para(
      t(
        'It combines the components as u = √(u(precision)² + u(bias)² + …), at each level, and multiplies by the coverage factor (k = 2 for about 95 %) to get the expanded uncertainty U. The results show u and U, absolute and relative, at every level.',
        'Bileşenleri her düzeyde u = √(u(kesinlik)² + u(sapma)² + …) olarak birleştirir ve genişletilmiş belirsizlik U\'yu bulmak için kapsam faktörüyle çarpar (yaklaşık %95 için k = 2). Sonuçlarda her düzeyde mutlak ve bağıl u ve U verilir.',
      ),
    );
    tip(
      t(
        'Compare the size of the precision and bias components in the results. If bias dominates, a better reference material or a correction helps more than repeating measurements.',
        'Sonuçlarda kesinlik ve sapma bileşenlerinin büyüklüğünü karşılaştırın. Sapma baskınsa ölçümleri tekrarlamaktan çok daha iyi bir referans malzeme ya da bir düzeltme işe yarar.',
      ),
    );
  }

  String _precisionData(Map d) {
    final type = d['type']?.toString() ?? '';
    final sd = d['sd'] ?? d['sd_percent'];
    final pct = d.containsKey('sd_percent') ? ' %' : '';
    if (sd is num) {
      return t(
        'a standard deviation of ${g(sd)}$pct (e.g. from a control chart: within-laboratory reproducibility).',
        'standart sapma ${g(sd)}$pct (örneğin bir kontrol grafiğinden: laboratuvar içi tekrar üretilebilirlik).',
      );
    }
    if (d['pairs'] is List) {
      return t(
        '${(d['pairs'] as List).length} duplicate pairs; the standard deviation follows from the differences within the pairs.',
        '${(d['pairs'] as List).length} çift ölçüm; standart sapma çiftlerin içindeki farklardan bulunur.',
      );
    }
    if (d['groups'] is List) {
      return t(
        '${(d['groups'] as List).length} groups of repeated results (e.g. days); an analysis of variance separates repeatability from between-day variation.',
        '${(d['groups'] as List).length} grup tekrarlı sonuç (örneğin günler); varyans analizi tekrarlanabilirliği günler arası değişimden ayırır.',
      );
    }
    if (d['sd_reproducibility'] != null || d['sd_reproducibility_percent'] != null) {
      return t(
        'repeatability and reproducibility from an interlaboratory study (ISO 5725 / ISO 21748).',
        'bir laboratuvarlar arası çalışmadan tekrarlanabilirlik ve tekrar üretilebilirlik (ISO 5725 / ISO 21748).',
      );
    }
    return type;
  }

  void _proficiency(Map pt) {
    final results = (pt['results'] as List?) ?? const [];
    h(t('Step 1. The idea: comparing laboratories', 'Adım 1. Fikir: laboratuvarları karşılaştırmak'));
    if (results.isNotEmpty) {
      para(
        t(
          'In a **proficiency test**, the same material is sent to ${results.length} laboratories, which all measure **${pt['quantity'] ?? ''}**. Each laboratory then gets a score that tells whether its result agrees with the assigned value.',
          '**Yeterlilik testinde** aynı malzeme ${results.length} laboratuvara gönderilir ve hepsi **${pt['quantity'] ?? ''}** ölçer. Sonra her laboratuvar, sonucunun atanan değerle uyuşup uyuşmadığını söyleyen bir puan alır.',
        ),
      );
      final av = pt['assigned_value'];
      final method = av is Map ? av['method']?.toString() : null;
      final how = switch (method) {
        'algorithm_a' => t(
          'the robust average of the participants\' results (Algorithm A, which limits the influence of outliers)',
          'katılımcı sonuçlarının dayanıklı ortalaması (aykırı değerlerin etkisini sınırlayan Algoritma A)',
        ),
        'median' => t('the median of the participants\' results', 'katılımcı sonuçlarının medyanı'),
        'mean' => t('the arithmetic mean of the participants\' results', 'katılımcı sonuçlarının aritmetik ortalaması'),
        'reference' => t(
          'a reference value determined independently (e.g. by a reference laboratory or from a certified reference material)',
          'bağımsız olarak belirlenmiş bir referans değer (örneğin bir referans laboratuvarınca ya da sertifikalı referans malzemeden)',
        ),
        _ => av is num ? g(av) : (method ?? ''),
      };
      para(
        t(
          'Assigned value: $how. The standard deviation for proficiency assessment σ_pt decides how far a result may be from the assigned value.',
          'Atanan değer: $how. Yeterlilik değerlendirmesi için standart sapma σ_pt, bir sonucun atanan değerden ne kadar uzak olabileceğine karar verir.',
        ),
      );
      h(t('Step 2. The scores', 'Adım 2. Puanlar'));
      item(
        t(
          '**z = (x − x_pt)/σ_pt**: |z| ≤ 2 satisfactory, 2 < |z| < 3 questionable, |z| ≥ 3 unsatisfactory.',
          '**z = (x − x_pt)/σ_pt**: |z| ≤ 2 başarılı, 2 < |z| < 3 şüpheli, |z| ≥ 3 başarısız.',
        ),
      );
      item(
        t(
          '**ζ and E_n** also use the uncertainty that each laboratory reported; they check whether the laboratory\'s own uncertainty is realistic.',
          '**ζ ve E_n**, her laboratuvarın bildirdiği belirsizliği de kullanır; laboratuvarın kendi belirsizliğinin gerçekçi olup olmadığını denetler.',
        ),
      );
      b.writeln();
    }
    if (pt['homogeneity'] != null) {
      para(
        t(
          '**Homogeneity**: several units of the material are measured in replicate. An analysis of variance separates the variation between units from the repeatability of the measurement; the between-unit standard deviation must be small compared with σ_pt.',
          '**Homojenlik**: malzemenin birkaç birimi tekrarlı ölçülür. Varyans analizi birimler arası değişimi ölçümün tekrarlanabilirliğinden ayırır; birimler arası standart sapma σ_pt\'ye göre küçük olmalıdır.',
        ),
      );
    }
    if (pt['stability'] != null) {
      para(
        t(
          '**Stability**: units measured at the start and at the end of the round must agree.',
          '**Kararlılık**: turun başında ve sonunda ölçülen birimler uyuşmalıdır.',
        ),
      );
    }
  }

  void _calibration(Map c) {
    final pts = (c['points'] as List?) ?? const [];
    final samples = (c['samples'] as List?) ?? const [];
    h(t('Step 1. The idea: a calibration curve', 'Adım 1. Fikir: bir kalibrasyon eğrisi'));
    para(
      t(
        'An instrument gives a response (here in ${c['response_unit'] ?? ''}) that is not directly the content of **${c['analyte'] ?? ''}**. It is calibrated with ${pts.length} reference mixtures of known content. Both the contents and the responses have uncertainties, so the fit (generalised least squares, ISO 6143) uses both.',
        'Bir cihaz, doğrudan **${c['analyte'] ?? ''}** içeriği olmayan bir yanıt verir (burada ${c['response_unit'] ?? ''}). Cihaz, içeriği bilinen ${pts.length} referans karışımla kalibre edilir. Hem içeriklerin hem yanıtların belirsizliği vardır; bu yüzden uydurma (genelleştirilmiş en küçük kareler, ISO 6143) ikisini birden kullanır.',
      ),
    );
    para(
      t(
        'Analysis function: **${c['function'] ?? ''}**. The program checks whether it fits the points (the weighted residuals and Γ should be small).',
        'Analiz fonksiyonu: **${c['function'] ?? ''}**. Program fonksiyonun noktalara uyup uymadığını denetler (ağırlıklı artıklar ve Γ küçük olmalıdır).',
      ),
    );
    if (samples.isNotEmpty) {
      h(t('Step 2. Unknown samples', 'Adım 2. Bilinmeyen numuneler'));
      para(
        t(
          'For ${samples.length} sample${samples.length == 1 ? '' : 's'}, the measured response is converted into a content through the fitted function; the uncertainty includes both the response of the sample and the uncertainty of the curve.',
          '${samples.length} numune için ölçülen yanıt, uydurulan fonksiyonla içeriğe dönüştürülür; belirsizlik hem numunenin yanıtını hem eğrinin belirsizliğini içerir.',
        ),
      );
    }
  }

  void _precision(Map pe) {
    final labs = (pe['laboratories'] as List?) ?? const [];
    final levels = (pe['levels'] as List?) ?? const [];
    h(t('Step 1. The idea: an interlaboratory precision study', 'Adım 1. Fikir: laboratuvarlar arası kesinlik çalışması'));
    para(
      t(
        '${labs.length} laboratories measured **${pe['quantity'] ?? ''}** in replicate at ${levels.length} levels (ISO 5725-2). From this, the program calculates the **repeatability** s_r (same laboratory, same day) and the **reproducibility** s_R (different laboratories) of the method.',
        '${labs.length} laboratuvar, **${pe['quantity'] ?? ''}** büyüklüğünü ${levels.length} düzeyde tekrarlı ölçtü (ISO 5725-2). Program bundan yöntemin **tekrarlanabilirliğini** s_r (aynı laboratuvar, aynı gün) ve **tekrar üretilebilirliğini** s_R (farklı laboratuvarlar) hesaplar.',
      ),
    );
    para(
      t(
        'Before that, outlier tests (Cochran for variances, Grubbs for means) and Mandel\'s h and k statistics show laboratories whose results do not fit. The method is ${pe['method'] == 'reml' ? 'REML (restricted maximum likelihood), which handles unbalanced data' : 'the classical analysis of variance of ISO 5725-2'}.',
        'Bundan önce aykırı değer testleri (varyanslar için Cochran, ortalamalar için Grubbs) ve Mandel h ve k istatistikleri uymayan laboratuvarları gösterir. Yöntem: ${pe['method'] == 'reml' ? 'REML (kısıtlı en çok olabilirlik); dengesiz verilerle başa çıkar' : 'ISO 5725-2\'nin klasik varyans analizi'}.',
      ),
    );
  }

  void _controlChart(Map cc) {
    h(t('Step 1. The idea: watching a method over time', 'Adım 1. Fikir: bir yöntemi zaman içinde izlemek'));
    para(
      t(
        'A **control chart** (Nordtest TR 569) plots the result of a control sample, measured with every series, against time. Warning limits (±2 s) and action limits (±3 s) around a central line show when the method is no longer in statistical control.',
        'Bir **kontrol grafiği** (Nordtest TR 569), her seriyle ölçülen bir kontrol numunesinin sonucunu zamana karşı çizer. Merkez çizgisinin etrafındaki uyarı (±2 s) ve eylem (±3 s) sınırları, yöntemin istatistiksel kontrolden ne zaman çıktığını gösterir.',
      ),
    );
    final limits = cc['limits'];
    if (limits is Map) {
      final basis = limits['basis']?.toString() ?? '';
      para(
        t(
          'Limits: ${basis == 'statistical'
              ? 'statistical, from the scatter of the control values themselves'
              : basis == 'target'
              ? 'target limits, from the precision the method must reach'
              : basis}.',
          'Sınırlar: ${basis == 'statistical'
              ? 'istatistiksel, kontrol değerlerinin kendi dağılımından'
              : basis == 'target'
              ? 'hedef sınırlar, yöntemin ulaşması gereken kesinlikten'
              : basis}.',
        ),
      );
    }
    final values = (cc['values'] as List?) ?? (cc['runs'] as List?) ?? const [];
    if (values.isNotEmpty) para(t('${values.length} control values are entered.', '${values.length} kontrol değeri girilmiş.'));
    tip(
      t(
        'In the results, look for points beyond the action limits and for two of three successive points beyond the warning limits: these are the rules that say "stop and find the cause".',
        'Sonuçlarda eylem sınırlarının dışındaki noktalara ve art arda üç noktadan ikisinin uyarı sınırlarının dışında kaldığı durumlara bakın: bunlar "dur ve nedeni bul" diyen kurallardır.',
      ),
    );
  }

  void _sampling(Map sp) {
    final dup = (sp['duplicates'] as List?) ?? const [];
    h(t('Step 1. The idea: the uncertainty starts at sampling', 'Adım 1. Fikir: belirsizlik numune almayla başlar'));
    para(
      t(
        'A result describes a whole lot, field or batch, but only a small sample is analysed. The **duplicate method** (Eurachem UfS) takes two independent samples from ${dup.isEmpty ? 'several' : dup.length} targets and analyses each twice. An analysis of variance then separates the variation due to **sampling** from that due to **analysis**.',
        'Bir sonuç bütün bir partiyi, tarlayı ya da yığını anlatır, ama yalnızca küçük bir numune analiz edilir. **Çift numune yöntemi** (Eurachem UfS), ${dup.isEmpty ? 'birkaç' : '${dup.length}'} hedeften ikişer bağımsız numune alır ve her birini iki kez analiz eder. Varyans analizi sonra **numune almadan** ve **analizden** gelen değişimi birbirinden ayırır.',
      ),
    );
    if (sp['gy'] != null) {
      para(
        t(
          'Here, the fundamental sampling error is also estimated from the properties of the particles (Gy\'s theory).',
          'Burada temel numune alma hatası da parçacıkların özelliklerinden kestirilir (Gy teorisi).',
        ),
      );
    }
    tip(
      t(
        'Compare u(sampling) with u(analysis) in the results: often the sampling part is the larger one, and a better analysis would not help.',
        'Sonuçlarda u(numune alma) ile u(analiz)\'i karşılaştırın: çoğu zaman numune alma payı daha büyüktür ve daha iyi bir analiz işe yaramaz.',
      ),
    );
  }

  void _errorFunction(Map ef) {
    final pts = (ef['points'] as List?) ?? const [];
    h(t('Step 1. The idea: calibrating a balance', 'Adım 1. Fikir: bir terazinin kalibrasyonu'));
    para(
      t(
        'The ${ef['instrument'] ?? 'weighing instrument'} is tested with standard weights at ${pts.length} loads (EURAMET cg-18). At each load the error E = indication − reference mass and its uncertainty are found (repeatability, rounding of the display, eccentricity, the weights, air buoyancy).',
        '${ef['instrument'] ?? 'Tartı aleti'}, ${pts.length} yükte standart ağırlıklarla denenir (EURAMET cg-18). Her yükte hata E = gösterge − referans kütle ve belirsizliği bulunur (tekrarlanabilirlik, gösterge yuvarlaması, eksantriklik, ağırlıklar, hava kaldırma kuvveti).',
      ),
    );
    para(
      t(
        'A straight line (the error approximation function) is fitted through the errors, so that the error and its uncertainty are known for every load, not only the tested ones.',
        'Hatalardan bir doğru (hata yaklaşım fonksiyonu) geçirilir; böylece hata ve belirsizliği yalnızca denenen yükler için değil, her yük için bilinir.',
      ),
    );
    if (ef['in_use'] != null) {
      para(
        t(
          'Then the **uncertainty of a weighing result in use** is calculated, including effects that occur in daily use (temperature, eccentricity, tare), and the **minimum weight**: the smallest load whose relative uncertainty is still acceptable.',
          'Sonra günlük kullanımda ortaya çıkan etkiler de (sıcaklık, eksantriklik, dara) katılarak **kullanımdaki bir tartım sonucunun belirsizliği** ve **minimum tartım**, yani bağıl belirsizliği hâlâ kabul edilebilir olan en küçük yük hesaplanır.',
        ),
      );
    }
  }

  void _consensus(Map cs) {
    final results = (cs['results'] as List?) ?? const [];
    h(t('Step 1. The idea: one value from several laboratories', 'Adım 1. Fikir: birkaç laboratuvardan tek bir değer'));
    para(
      t(
        '${results.length} laboratories or experiments measured **${cs['measurand'] ?? ''}**, each with its own uncertainty. Often the results disagree by more than their uncertainties allow: there is an unexplained spread between laboratories (*dark uncertainty*). A **consensus value** combines them while accounting for that spread.',
        '${results.length} laboratuvar ya da deney **${cs['measurand'] ?? ''}** büyüklüğünü kendi belirsizliğiyle ölçtü. Sonuçlar çoğu zaman belirsizliklerinin izin verdiğinden fazla birbirinden ayrılır: laboratuvarlar arasında açıklanamayan bir yayılma vardır (*karanlık belirsizlik*). **Uzlaşı değeri** bu yayılmayı hesaba katarak sonuçları birleştirir.',
      ),
    );
    para(
      t(
        'Several procedures are available (DerSimonian–Laird, Linear Pool, hierarchical Bayes, maximum likelihood); they make different assumptions and can give different values. The **degrees of equivalence** D_j = x_j − x_ref show how far each laboratory is from the consensus value, with an uncertainty.',
        'Birkaç yöntem var (DerSimonian–Laird, Linear Pool, hiyerarşik Bayes, en çok olabilirlik); farklı varsayımlar yaparlar ve farklı değerler verebilirler. **Eşdeğerlik dereceleri** D_j = x_j − x_ref her laboratuvarın uzlaşı değerinden ne kadar uzak olduğunu, belirsizliğiyle birlikte gösterir.',
      ),
    );
    tip(
      t(
        'Switch procedures in the **Data** tab and compare the consensus values and the number of laboratories whose degree of equivalence is significant.',
        '**Veriler** sekmesinde yöntemleri değiştirip uzlaşı değerlerini ve eşdeğerlik derecesi anlamlı çıkan laboratuvar sayısını karşılaştırın.',
      ),
    );
  }

  void _multicomponent(Map mc) {
    final comps = (mc['components'] as List?) ?? const [];
    h(t('Step 1. The idea: several properties must conform at once', 'Adım 1. Fikir: birkaç özellik aynı anda uymalı'));
    para(
      t(
        'An item conforms only if all ${comps.length} components are within their limits. Because the measured values are correlated, the risks of a wrong decision are calculated jointly (Monte Carlo).',
        'Bir ürün ancak ${comps.length} bileşenin hepsi sınırları içindeyse uygundur. Ölçülen değerler korelasyonlu olduğu için yanlış karar riskleri birlikte hesaplanır (Monte Carlo).',
      ),
    );
  }

  void _conformity(Map cf, {required bool standalone}) {
    h(
      standalone
          ? t('Step 1. The idea: does the item meet the specification?', 'Adım 1. Fikir: ürün spesifikasyona uyuyor mu?')
          : t('Then: does the result meet the specification?', 'Ardından: sonuç spesifikasyona uyuyor mu?'),
    );
    final lo = cf['lower'];
    final hi = cf['upper'];
    final limits = lo != null && hi != null
        ? t('between ${g(lo)} and ${g(hi)}', '${g(lo)} ile ${g(hi)} arasında')
        : hi != null
        ? t('at most ${g(hi)}', 'en çok ${g(hi)}')
        : t('at least ${g(lo)}', 'en az ${g(lo)}');
    para(
      t(
        'The measured quantity${cf['quantity'] == null ? '' : ' ${cf['quantity']}'} must be $limits${unit(cf['unit'])}. Because every measured value is uncertain, a result close to a limit could be on either side of it. The program calculates the **probability of conformity** and applies a **decision rule** (JCGM 106).',
        'Ölçülen büyüklük${cf['quantity'] == null ? '' : ' ${cf['quantity']}'} $limits${unit(cf['unit'])} olmalıdır. Her ölçülen değer belirsiz olduğundan sınıra yakın bir sonuç sınırın her iki tarafında da olabilir. Program **uygunluk olasılığını** hesaplar ve bir **karar kuralı** uygular (JCGM 106).',
      ),
    );
    final rule = cf['rule']?.toString();
    if (rule != null) {
      final what = switch (rule) {
        'guarded_acceptance' => t(
          'guarded acceptance: the acceptance zone is made smaller than the tolerance by a guard band, so that accepting a bad item is unlikely (protects the customer)',
          'korumalı kabul: kabul bölgesi tolerans aralığından bir koruma bandı kadar daraltılır; böylece kötü bir ürünü kabul etmek olasılık dışı olur (müşteriyi korur)',
        ),
        'guarded_rejection' => t(
          'guarded rejection: an item is rejected only if it is clearly outside the limit (e.g. in law enforcement or anti-doping, where a false accusation must be avoided)',
          'korumalı ret: bir ürün ancak sınırın açıkça dışındaysa reddedilir (örneğin haksız bir suçlamadan kaçınılması gereken yasal denetim ya da doping kontrolünde)',
        ),
        'simple_acceptance' => t('simple acceptance: accept if the measured value is inside the limits', 'basit kabul: ölçülen değer sınırlar içindeyse kabul'),
        _ => rule,
      };
      para(t('Decision rule: $what.', 'Karar kuralı: $what.'));
    }
    if (cf['process'] != null) {
      para(
        t(
          'Information about the production process (the spread of items before measurement) is also used, so that the **global risks** can be calculated: the fraction of all accepted items that are actually bad (consumer\'s risk) and of rejected items that are actually good (producer\'s risk).',
          'Üretim süreci hakkındaki bilgi (ölçümden önce ürünlerin dağılımı) de kullanılır; böylece **genel riskler** hesaplanabilir: kabul edilen tüm ürünlerden gerçekte kötü olanların oranı (müşteri riski) ve reddedilenlerden gerçekte iyi olanların oranı (üretici riski).',
        ),
      );
    }
  }

  // --- Expected results ------------------------------------------------------------------------------------------

  void _expected() {
    final e = p['expected'] as Map?;
    if (e == null) return;
    h(t('Finally: what the source expects', 'Son olarak: kaynak ne sonuç veriyor?'));
    final gum = e['gum'] as Map?;
    if (gum != null) {
      final outUnit = unit(((p['model'] as Map?)?['output'] as Map?)?['unit']);
      String v(String key) => gum[key] is Map ? '${g((gum[key] as Map)['value'])}${key == 'coverage_factor' ? '' : outUnit}' : '';
      final rows = <String>[];
      if (v('estimate').isNotEmpty) rows.add(t('estimate y = ${v('estimate')}', 'tahmin y = ${v('estimate')}'));
      if (v('standard_uncertainty').isNotEmpty) {
        rows.add(t('standard uncertainty u = ${v('standard_uncertainty')}', 'standart belirsizlik u = ${v('standard_uncertainty')}'));
      }
      if (v('coverage_factor').isNotEmpty) rows.add(t('coverage factor k = ${v('coverage_factor')}', 'kapsam faktörü k = ${v('coverage_factor')}'));
      if (v('expanded_uncertainty').isNotEmpty) {
        rows.add(t('expanded uncertainty U = ${v('expanded_uncertainty')}', 'genişletilmiş belirsizlik U = ${v('expanded_uncertainty')}'));
      }
      if (rows.isNotEmpty) {
        para(t('With the GUM method the source gives:', 'GUM yöntemiyle kaynak şunları verir:'));
        for (final r in rows) {
          item(r);
        }
        b.writeln();
      }
    }
    final ref = e['reference'];
    if (ref != null) para('${t('Reference', 'Kaynak')}: ${text(ref)}');
    para(
      t(
        'When you press **Calculate**, the program compares its own results with these values and marks each one as within or outside its tolerance (the tolerance allows for the rounding of the printed numbers). Any difference that the source cannot explain is described under **Notes**.',
        '**Hesapla**\'ya bastığınızda program kendi sonuçlarını bu değerlerle karşılaştırır ve her birini toleransı içinde ya da dışında olarak işaretler (tolerans, basılı sayıların yuvarlanmasını hesaba katar). Kaynağın açıklayamadığı bir fark varsa **Notlar** bölümünde anlatılır.',
      ),
    );
  }
}
